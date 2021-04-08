#!/usr/bin/env python3
import pandas as pd
import ratelimit
import requests

from common import *

pd.set_option('display.max_colwidth', 10000)

# As per https://crates.io/policies
#   > limit their request rate to 1 request per second or less
# So we limit ourselves to 1 request every 2 seconds.
@ratelimit.sleep_and_retry
@ratelimit.limits(calls=30, period=60)
def get_downloads_for_crate_from_cargo_api(crate):
    headers = {'User-Agent': 'Download-Stats (github.com/ammaraskar)'}

    r = requests.get("https://crates.io/api/v1/crates/{}".format(crate), headers=headers)
    r.raise_for_status()

    return r.json()['crate']['downloads']

def fetch_missing_download_counts(row):
    if pd.isnull(row['Downloads']):
        return int(get_downloads_for_crate_from_cargo_api(row['Crate']))
    return row['Downloads']

def main():
    metadata = pd.read_csv(PROJECT_DIRECTORY / 'paper' / 'metadata.csv')

    # Pad poc ids to 4-digits with leading zeroes. 8 -> 0008
    metadata['ID'] = metadata['ID'].apply(lambda x: "{:04d}".format(x))

    # Fetch any missing download entries and then save the csv back.
    metadata['Downloads'] = metadata.apply(fetch_missing_download_counts, axis=1)
    metadata.to_csv(PROJECT_DIRECTORY / 'paper' / 'metadata.csv', index=False)

    # Split multiple locations
    metadata['Bug Location'] = metadata['Bug Location'].fillna(value='')
    metadata['Bug Location'] = metadata['Bug Location'].apply(lambda x: x.split(';'))

    # Split multiple latent times
    metadata['L'] = metadata['L'].fillna(value='--')
    metadata['L'] = metadata['L'].apply(lambda x: x.split(';'))

    # Drop the Comment column, it's only for humans to add comments in the
    # metadata table.
    metadata = metadata.drop(columns=['Comment'])

    poc_metadata = get_poc_metadata()
    rustsec_metadata = get_rustsec_metadata()

    # Drop any purely manually found bugs.
    purely_manual_pocs = set([
        id
        for id, poc in poc_metadata.items()
        if all(map(lambda bug: bug['analyzer'] == 'Manual', poc['bugs']))
    ])
    metadata = metadata[~metadata['ID'].isin(purely_manual_pocs)]

    # Column for algorithm used.
    metadata['Algorithm'] = metadata['ID'].apply(get_bug_algorithm,
        poc_metadata=poc_metadata)

    # Add a column containing bug identifiers for the crates
    metadata['Bug Identifiers'] = metadata.apply(get_bug_identifiers, axis=1,
        poc_metadata=poc_metadata, rustsec_metadata=rustsec_metadata)

    metadata['Has Unit Tests'] = metadata['Unit Test Coverage'].apply(
        lambda cov: True if not pd.isnull(cov) and cov > 50 else False)

    # Only do the first 25 bugs for now
    metadata = metadata.head(25)

    # Manually put in the std library bugs.
    std_bug = {
        'Crate': ['std', 'rustc'],
        'Bug Location': [['str.rs', 'mod.rs'], ['worker_local.rs']],
        'Algorithm': [['UnsafeDataflow'], ['SendSyncVariance']],
        'Bug Identifiers': [['rust#80335', 'rust#80894'], ['rust#81425']],
        # Computed with:
        #   cloc ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std
        'Size (LoC)': [60836, 347739],
        'Unsafe Uses': [1925, 1574],
        'L': [['3y', '2y'], ['3y']],
        # Assume that the stdlib has good coverage, afaik measuring this with tools is hard.
        'Has Unit Tests': [True, True],
        'Description': [
            r'The \texttt{join} method can return uninitialized memory when string length changes. '
            r'\texttt{read_to_string} and \texttt{read_to_end} methods '
            r'overflow the heap and read past the provided buffer. ',

            r'\texttt{WorkerLocal} used in parallel compilation can cause data races.'],
    }
    metadata = pd.concat([pd.DataFrame.from_dict(std_bug), metadata])

    print_table(metadata)

def format_list_for_latex_table(pandas_list):
    if len(pandas_list) == 0:
        return '--'
    if len(pandas_list) == 1:
        return pandas_list[0]

    # Note that we use a special identifier here to avoid panda's auto escaping,
    # this will get fixed afterwards
    with_latex_newlines = 'ReplaceWithDoubleBackslash'.join(pandas_list)
    return 'ReplaceWithMakeCell' + with_latex_newlines + 'ReplaceWithEndCurly'

ALGORITHM_NAMES_SHORT = {
    'PanicSafety': 'PS',
    'SendSyncVariance': 'SV',
    'UnsafeDataflow': 'UD',
    'UnsafeDestructor': 'D',
}
def format_algorithm_names(algos):
    without_manual = [a for a in algos if a != 'Manual']
    short_names = [ALGORITHM_NAMES_SHORT[a] for a in without_manual]

    cell = '/'.join(short_names)
    #if 'Manual' in algos:
    #    cell += 'ReplaceWithDagger'
    return cell

# Turn numbers into stuff like 10k, 5M, 101k, 200 etc.
def format_number_abreviation(x, round_hundreds=True):
    if pd.isnull(x):
        return "--"

    if x > 1_000_000:
        return "{}M".format(int(round(x, -6) / 1_000_000))
    elif x > 1_000:
        return "{}K".format(int(round(x, -3) / 1_000))
    elif x > 100 and round_hundreds:
        # Round to nearest hundrendth
        return str(int(x / 100) * 100)
    return str(int(x))

def append_extra_bug_identifiers(row):
    if not pd.isnull(row['Extra Bug Identifiers']):
        row['Bug Identifiers'].append(row['Extra Bug Identifiers'])
    return row['Bug Identifiers']

def print_table(table):
    # Add dagger to crate name if it has fuzzer
    table['Crate'] = table.apply(
        lambda row: row['Crate'] + 'ReplaceWithDagger' if row['Has Fuzzer'] == 'Y' else row['Crate'],
        axis=1)

    # Contract "RUSTSEC-" to "RSC-" in bug identifiers.
    table['Bug Identifiers'] = table['Bug Identifiers'].apply(
        lambda bug_list: [x.replace('RUSTSEC-', 'R-').replace('CVE-', 'C-') for x in bug_list])

    table['Bug Identifiers'] = table.apply(append_extra_bug_identifiers, axis=1)

    # Apply any formatting touches and print the table.
    table['Bug Location'] = table['Bug Location'].apply(format_list_for_latex_table)
    table['Bug Identifiers'] = table['Bug Identifiers'].apply(format_list_for_latex_table)
    table['L'] = table['L'].apply(format_list_for_latex_table)

    # Perform some convenience replacements to use LaTeX in description.
    table['Description'] = table['Description'].fillna(value='')
    table['Description'] = table['Description'].apply(
        lambda desc: desc.replace(r'\texttt{', 'ReplaceWithTextTTT')
                         .replace(r'}', 'ReplaceWithEndCurly')
    )

    table['Downloads'] = table['Downloads'].apply(format_number_abreviation)
    # Round LoC to nearest hundred
    table['Size (LoC)'] = table['Size (LoC)'].apply(format_number_abreviation)
    table['Unsafe Uses'] = table['Unsafe Uses'].apply(
        format_number_abreviation, round_hundreds=False)

    # Drop the ID column
    table = table.drop(columns=['ID'])

    # Use short names for the algorithm column
    table['Algorithm'] = table['Algorithm'].apply(format_algorithm_names)
    # Make unit test columns checkmark.
    table['Has Unit Tests'] = table['Has Unit Tests'].apply(
        lambda u: 'ReplaceWithCheckMark' if u else '')

    # Abbreviate some column names.
    table = table.rename(columns={
        'Bug Location': 'Location',
        'Downloads': 'DLs',
        'Size (LoC)': 'LoC',
        'Algorithm': 'Algorithm Used',
        'L': 'Latent Time',
    })

    as_latex = table.to_latex(na_rep='--', index=False,
        column_format = 'llcrrlp{7.6cm}rl',
        columns = [
            'Crate', 'Location', 'Has Unit Tests', 'LoC', 'Unsafe Uses',
            'Algorithm Used', 'Description', 'Latent Time', 'Bug Identifiers'
        ]
    )

    # Add little footnotes to the contracted columns.
    as_latex = as_latex.replace('Has Unit Tests', r'T\textsuperscript{$1$}')
    as_latex = as_latex.replace('LoC', r'LoC\textsuperscript{$2$}')
    as_latex = as_latex.replace('Unsafe Uses', r'UU\textsuperscript{$3$}')
    as_latex = as_latex.replace('Algorithm Used', r'Alg\textsuperscript{$4$}')
    as_latex = as_latex.replace('Latent Time', r'L\textsuperscript{$5$}')

    as_latex = as_latex.replace('ReplaceWithDoubleBackslash', r'\\')
    as_latex = as_latex.replace('ReplaceWithMakeCell', r'\makecell[tl]{')
    as_latex = as_latex.replace('ReplaceWithEndCurly', r'}')
    as_latex = as_latex.replace('ReplaceWithDagger', r'$^\dagger$')
    as_latex = as_latex.replace('ReplaceWithTextTTT', r'\texttt{')
    as_latex = as_latex.replace('ReplaceWithCheckMark', r'\checkmark')
    print(as_latex)

if __name__ == '__main__':
    main()
