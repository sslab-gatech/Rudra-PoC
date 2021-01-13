#!/usr/bin/env python3
import tomlkit
import pandas as pd
import ratelimit
import requests

from pathlib import Path

pd.set_option('display.max_colwidth', 10000)
PROJECT_DIRECTORY = Path(__file__).resolve().parent.parent


# As per https://crates.io/policies
#   > limit their request rate to 1 request per second or less
# So we limit ourselves to 1 request every 2 seconds.
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


RUSTSEC_FRONTMATTER = '```toml'

def get_frontmatter(file, header):
    contents = file.read()
    frontmatter_start = contents.index(header)
    frontmatter_end = contents.index('```', frontmatter_start + 1)

    metadata = contents[frontmatter_start + len(header):frontmatter_end]
    return tomlkit.parse(metadata)


# Returns a dict of RUSTSEC ids -> RUSTSEC metadata.
def get_rustsec_metadata():
    rustsec_metadata = {}

    rustsec_dir = PROJECT_DIRECTORY / 'advisory-db' / 'crates'
    for advisory_file in rustsec_dir.glob('**/*.md'):
        with advisory_file.open() as f:
            metadata = get_frontmatter(f, RUSTSEC_FRONTMATTER)['advisory']
            rustsec_metadata[metadata['id']] = metadata

    return rustsec_metadata

POC_FRONTMATTER = '```rudra-poc'

# Returns a dict of POC ids -> POC metadata
def get_poc_metadata():
    poc_metadata = {}
    poc_dir = PROJECT_DIRECTORY / 'poc'

    for poc_file in poc_dir.iterdir():
        try:
            identifier = int(poc_file.stem.split('-')[0])
        except ValueError:
            continue

        with poc_file.open() as f:
            metadata = get_frontmatter(f, POC_FRONTMATTER)
            poc_metadata[identifier] = metadata

    return poc_metadata

def get_bug_algorithm(poc_id, poc_metadata):
    return poc_metadata[poc_id]['test']['analyzers']

def get_bug_identifiers(row, poc_metadata, rustsec_metadata):
    identifiers = []

    poc_metadata = poc_metadata[row['ID']]
    if 'rustsec_id' in poc_metadata['report']:
        rustsec_id = poc_metadata['report']['rustsec_id']
        rustsec_metadata = rustsec_metadata[rustsec_id]

        identifiers.append(rustsec_id)
        if 'aliases' in rustsec_metadata:
            identifiers.extend(rustsec_metadata['aliases'])
        
    return identifiers

def main():
    metadata = pd.read_csv(PROJECT_DIRECTORY / 'paper' / 'metadata.csv')

    # Fetch any missing download entries and then save the csv back.
    metadata['Downloads'] = metadata.apply(fetch_missing_download_counts, axis=1)
    metadata.to_csv(PROJECT_DIRECTORY / 'paper' / 'metadata.csv', index=False)

    # Split multiple locations
    metadata['Bug Location'] = metadata['Bug Location'].fillna(value='')
    metadata['Bug Location'] = metadata['Bug Location'].apply(lambda x: x.split(';'))

    # Split multiple latent times
    metadata['L'] = metadata['L'].fillna(value='--')
    metadata['L'] = metadata['L'].apply(lambda x: x.split(';'))

    # Split multiple descriptions
    metadata['Description'] = metadata['Description'].fillna(value='--')
    metadata['Description'] = metadata['Description'].apply(lambda x: x.split(';'))

    # Drop the Comment column, it's only for humans to add comments in the
    # metadata table.
    metadata = metadata.drop(columns=['Comment'])

    poc_metadata = get_poc_metadata()
    rustsec_metadata = get_rustsec_metadata()

    # Drop any purely manually found bugs.
    purely_manual_pocs = set([
        id
        for id, poc in poc_metadata.items()
        if poc['test']['analyzers'] == ['Manual']
    ])
    metadata = metadata[~metadata['ID'].isin(purely_manual_pocs)]

    # Column for algorithm used.
    metadata['Algorithm'] = metadata['ID'].apply(get_bug_algorithm,
        poc_metadata=poc_metadata)

    # Add a column containing bug identifiers for the crates
    metadata['Bug Identifiers'] = metadata.apply(get_bug_identifiers, axis=1,
        poc_metadata=poc_metadata, rustsec_metadata=rustsec_metadata)

    # Only do the first 35 bugs for now
    metadata = metadata.head(35)

    # Manually put in the std library bugs.
    std_bug = {
        'Crate': ['std'],
        'Bug Location': [['str.rs', 'mod.rs']],
        'Algorithm': [['PanicSafety']],
        'Bug Identifiers': [['rust-lang/rust#80335', 'rust-lang/rust#80894']],
        # Computed with:
        #   cloc ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library
        'Size (LoC)': [282518],
        'L': [['3y', '2y']],
        'Description': [['Uninit memory read in join']],
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
    'D': 'UnsafeDestructor'
}
def format_algorithm_names(algos):
    without_manual = [a for a in algos if a != 'Manual']
    short_names = [ALGORITHM_NAMES_SHORT[a] for a in without_manual]

    cell = '/'.join(short_names)
    if 'Manual' in algos:
        cell += 'ReplaceWithDagger'
    return cell

# Turn numbers into stuff like 10k, 5M, 101k, 200 etc.
def format_number_abreviation(x):
    if pd.isnull(x):
        return "--"

    if x > 1_000_000:
        return "{}M".format(int(x / 1_000_000))
    elif x > 1_000:
        return "{}K".format(int(x / 1_000))
    elif x > 100:
        # Round to nearest hundrendth
        return str(int(x / 100) * 100)
    return str(int(x))

def print_table(table):
    # Contract "RUSTSEC-" to "RSC-" in bug identifiers.
    table['Bug Identifiers'] = table['Bug Identifiers'].apply(
        lambda bug_list: [x.replace('RUSTSEC-', 'RSC-') for x in bug_list])

    # Apply any formatting touches and print the table.
    table['Bug Location'] = table['Bug Location'].apply(format_list_for_latex_table)
    table['Bug Identifiers'] = table['Bug Identifiers'].apply(format_list_for_latex_table)
    table['L'] = table['L'].apply(format_list_for_latex_table)
    table['Description'] = table['Description'].apply(format_list_for_latex_table)

    table['Downloads'] = table['Downloads'].apply(format_number_abreviation)
    # Round LoC to nearest hundred
    table['Size (LoC)'] = table['Size (LoC)'].apply(format_number_abreviation)
    #table['Size (LoC)'] = table['Size (LoC)'].apply(lambda x: '{:,.0f}'.format(int(x / 100) * 100))

    # Drop the ID column
    table = table.drop(columns=['ID'])

    # Use short names for the algorithm column
    table['Algorithm'] = table['Algorithm'].apply(format_algorithm_names)

    # Abbreviate some column names.
    table = table.rename(columns={
        'Downloads': 'DLs',
        'Size (LoC)': 'LoC',
        'Algorithm': 'Algo'
    })

    as_latex = table.to_latex(na_rep='--', index=False,
        column_format = 'llrrlp{7cm}rl',
        columns = [
            'Crate', 'Bug Location', 'DLs', 'LoC',
            'Algo', 'Description', 'L', 'Bug Identifiers'
        ]
    )
    as_latex = as_latex.replace('ReplaceWithDoubleBackslash', r'\\')
    as_latex = as_latex.replace('ReplaceWithMakeCell', r'\makecell[tl]{')
    as_latex = as_latex.replace('ReplaceWithEndCurly', r'}')
    as_latex = as_latex.replace('ReplaceWithDagger', r'$^\dagger$')
    print(as_latex)

if __name__ == '__main__':
    main()
