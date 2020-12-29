use std::collections::HashMap;
use std::fs;
use std::io::Read;

use crate::poc::PocMap;
use crate::prelude::*;

use flate2::read::GzDecoder;
use reqwest;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json;
use structopt::StructOpt;

pub const CARGO_INFO_FILE: &str = "cargo-tally.json.gz";
pub const CARGO_TALLY_DOWNLOAD_URL: &str =
    "https://github.com/dtolnay/cargo-tally/releases/download/2020-11-25/tally.json.gz";

fn download_tally_file_if_does_not_exist() -> Result<()> {
    let cargo_info_file_path = PROJECT_PATH.join(CARGO_INFO_FILE);
    if cargo_info_file_path.exists() {
        return Ok(());
    }

    eprintln!("Downloading cargo.io data");
    let mut output_file = fs::File::create(cargo_info_file_path)?;

    let mut jsonz = reqwest::blocking::get(CARGO_TALLY_DOWNLOAD_URL)?.error_for_status()?;
    jsonz.copy_to(&mut output_file)?;
    Ok(())
}

fn get_decompressed_cargo_data() -> Result<String> {
    let file = fs::File::open(PROJECT_PATH.join(CARGO_INFO_FILE))?;
    let mut decoder = GzDecoder::new(file);

    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)?;

    Ok(decompressed)
}

#[derive(Serialize, Deserialize)]
struct Crate {
    name: String,
    #[serde(rename = "vers")]
    version: Version,
    #[serde(rename = "deps")]
    dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize)]
struct Dependency {
    pub name: String,
    pub req: VersionReq,
}

fn get_crates_unreported_to_rustsec() -> Result<HashMap<String, Version>> {
    let mut unreported_crates = HashMap::new();

    let poc_map = PocMap::new()?;
    for poc_id in poc_map.iter_ids() {
        let metadata = poc_map.read_metadata(poc_id)?;

        if !metadata.report.rustsec_id.is_some() {
            unreported_crates.insert(metadata.target.krate, metadata.target.version);
        }
    }
    Ok(unreported_crates)
}

struct CrateStatistics {
    name: String,
    num_dependents: usize,
}

/// Takes a Map of crate names to their versions and returns a vector of
/// statistics computed for them.
fn get_statistics_for_crates(crates: HashMap<String, Version>) -> Result<Vec<CrateStatistics>> {
    let mut num_dependent_crates = HashMap::<String, usize>::new();

    let decompressed = get_decompressed_cargo_data()?;
    let de = serde_json::Deserializer::from_str(&decompressed);

    for line in de.into_iter::<Crate>() {
        let krate = line?;
        for dependency in krate.dependencies {
            // Is this dependency on of our crates? And let's see if its version
            // matches the VersionReq or if its unaffected.
            let version = crates.get(&dependency.name);
            if version.is_some() && dependency.req.matches(version.unwrap()) {
                let count = num_dependent_crates.entry(dependency.name).or_insert(0);
                *count += 1;
            }
        }
    }

    Ok(num_dependent_crates
        .iter()
        .map(|(krate_name, dependents)| CrateStatistics {
            name: krate_name.clone(),
            num_dependents: *dependents,
        })
        .collect())
}

#[derive(Debug, StructOpt)]
pub struct PrioritizeArgs {}

pub fn cmd_prioritize(_args: PrioritizeArgs) -> Result<()> {
    download_tally_file_if_does_not_exist()?;

    let unreported_crates = get_crates_unreported_to_rustsec()?;
    let mut crate_statistics = get_statistics_for_crates(unreported_crates)?;

    crate_statistics.sort_unstable_by(|a, b| a.num_dependents.cmp(&b.num_dependents));
    for krate in crate_statistics {
        println!("{},{}", krate.name, krate.num_dependents);
    }

    Ok(())
}
