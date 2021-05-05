import tomlkit

from collections import defaultdict
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
        identifier = poc_file.stem[:4]
        if not (poc_file.stem[4] == '-' and poc_file.stem[:4].isnumeric()):
            continue

        with poc_file.open() as f:
            metadata = get_frontmatter(f, POC_FRONTMATTER)
            poc_metadata[identifier] = metadata

    return poc_metadata

def get_poc_detailed_metadata():
    """Scan pocs to collect (bug_type, crate_id, bug_span)"""
    poc_metadata = get_poc_metadata()
    
    metadata = {
        # crate_id => set(bug_span)
        'SendSyncVariance': defaultdict(lambda: set()),
        # crate_id => set(bug_span)
        'UnsafeDataflow': defaultdict(lambda: set()),
    }

    for key, item in poc_metadata.items():
        crate_key = 'indexed_name' if 'indexed_name' in item['target'] else 'crate'
        version_key = 'indexed_version' if 'indexed_version' in item['target'] else 'version'
        crate_id = f"{item['target'][crate_key]}-{item['target'][version_key]}"
        
        for bug in filter(lambda x: x['analyzer'] in ['SendSyncVariance', 'UnsafeDataflow'], item['bugs']):
            for bug_span in bug['rudra_report_locations']:
                metadata[bug['analyzer']][crate_id].add(bug_span)

    return metadata

def get_unreported_metadata():
    """Scan unreported bugs to collect (bug_type, crate_id, bug_span)"""
    unreported_dir = PROJECT_DIRECTORY / 'unreported'

    metadata = {
        # crate_id => dict(bug_span => is_internal)
        'SendSyncVariance': defaultdict(lambda: dict()),
        # crate_id => dict(bug_span => is_internal)
        'UnsafeDataflow': defaultdict(lambda: dict()),
    }

    for unreported_file in unreported_dir.iterdir():
        with unreported_file.open() as f:
            report_file_str = f.read()
            report_dict = tomlkit.loads(report_file_str)
            
            version_key = 'indexed_version' if 'indexed_version' in report_dict['target'] else 'version'
            crate_id = f"{report_dict['target']['crate']}-{report_dict['target'][version_key]}"
            
            for bug in report_dict['bugs']:
                if bug['analyzer'] in ['UnsafeDataflow', 'SendSyncVariance']:
                    metadata[bug['analyzer']][crate_id][bug['location']] = (bug['reason'] == 'internal')
    
    return metadata

def get_bug_algorithm(poc_id, poc_metadata):
    return list(map(lambda bug: bug['analyzer'], poc_metadata[poc_id]['bugs']))


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
