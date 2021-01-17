import tomlkit

from pathlib import Path

PROJECT_DIRECTORY = Path(__file__).resolve().parent.parent

RUSTSEC_FRONTMATTER = '```toml'
POC_FRONTMATTER = '```rudra-poc'

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
