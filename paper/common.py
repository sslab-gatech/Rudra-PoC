import re
import tomlkit

from pathlib import Path

PROJECT_DIRECTORY = Path(__file__).resolve().parent.parent

RUSTSEC_FRONTMATTER = "```toml"
POC_FRONTMATTER = "```rudra-poc"


def get_frontmatter(contents, header):
    frontmatter_start = contents.index(header)
    frontmatter_end = contents.index("```", frontmatter_start + 1)

    metadata = contents[frontmatter_start + len(header) : frontmatter_end]
    return tomlkit.parse(metadata)


def get_rustsec_title(contents):
    frontmatter_end = contents.index("\n```\n")

    title_start = contents.index("\n# ", frontmatter_end + 1) + 3
    title_end = contents.index("\n", title_start)

    return contents[title_start:title_end]


# Returns a dict of RUSTSEC ids -> RUSTSEC metadata.
def get_rustsec_metadata():
    rustsec_metadata = {}

    rustsec_dir = PROJECT_DIRECTORY / "advisory-db" / "crates"
    for advisory_file in rustsec_dir.glob("**/*.md"):
        with advisory_file.open() as f:
            contents = f.read()
            metadata = get_frontmatter(contents, RUSTSEC_FRONTMATTER)["advisory"]
            metadata["title"] = get_rustsec_title(contents)
            rustsec_metadata[metadata["id"]] = metadata

    return rustsec_metadata


# Returns a dict of POC ids -> POC metadata
def get_poc_metadata():
    poc_metadata = {}
    poc_dir = PROJECT_DIRECTORY / "poc"

    for poc_file in poc_dir.iterdir():
        identifier = poc_file.stem[:4]
        if not (poc_file.stem[4] == "-" and poc_file.stem[:4].isnumeric()):
            continue

        with poc_file.open() as f:
            contents = f.read()
            metadata = get_frontmatter(contents, POC_FRONTMATTER)
            poc_metadata[identifier] = metadata

    return poc_metadata


# Returns a dict of crate name -> Unreported metadata
def get_unreported_metadata():
    unreported_metadata = {}
    unreported_dir = PROJECT_DIRECTORY / "unreported"

    for metadata_file in unreported_dir.iterdir():
        crate_name = metadata_file.stem

        with metadata_file.open() as f:
            metadata = tomlkit.parse(f.read())
            unreported_metadata[crate_name] = metadata

    return unreported_metadata


def get_bug_algorithm(poc_id, poc_metadata):
    return list(map(lambda bug: bug["analyzer"], poc_metadata[poc_id]["bugs"]))


def get_bug_identifiers(row, poc_metadata, rustsec_metadata):
    identifiers = []

    poc_metadata = poc_metadata[row["ID"]]
    if "rustsec_id" in poc_metadata["report"]:
        rustsec_id = poc_metadata["report"]["rustsec_id"]
        rustsec_metadata = rustsec_metadata[rustsec_id]

        identifiers.append(rustsec_id)
        if "aliases" in rustsec_metadata:
            identifiers.extend(rustsec_metadata["aliases"])

    return identifiers


ansi_escape_8bit = re.compile(
    r"(?:\x1B[@-Z\\-_]|[\x80-\x9A\x9C-\x9F]|(?:\x1B\[|\x9B)[0-?]*[ -/]*[@-~])"
)
