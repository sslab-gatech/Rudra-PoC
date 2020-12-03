use std::{collections::HashMap, fs, path::PathBuf};

use crate::prelude::*;

use once_cell::sync::Lazy;
use semver::Version;
use serde::{Deserialize, Serialize};
use toml::value::Datetime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub target: TargetMetadata,
    pub test: TestMetadata,
    pub report: ReportMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetMetadata {
    #[serde(rename = "crate")]
    pub krate: String,
    pub version: Version,
    #[serde(default)]
    pub peer: Vec<PeerMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerMetadata {
    #[serde(rename = "crate")]
    pub krate: String,
    pub version: Version,
    #[serde(default)]
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Analyzer {
    #[serde(rename = "manual")]
    Manual,
    UnsafeDestructor,
    SendSyncChecker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestMetadata {
    pub analyzers: Vec<Analyzer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub issue_url: Option<String>,
    pub issue_date: Option<Datetime>,
    pub rustsec_url: Option<String>,
    pub rustsec_id: Option<String>,
}

static METADATA_HEADER: &str = "/*!\n```rudra-poc\n";
static METADATA_FOOTER: &str = "```\n";

struct PocData {
    name: String,
    path: PathBuf,
}

pub struct PocMap(HashMap<PocId, PocData>);

impl PocMap {
    pub fn new() -> Result<Self> {
        let mut id_set = HashMap::new();

        let poc_dir = PROJECT_PATH.join("poc");
        for entry in fs::read_dir(&poc_dir)
            .with_context(|| format!("Failed to access {}", poc_dir.display()))?
        {
            static PATTERN: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"^((\d{4})-[A-Za-z0-9\-_]+)\.rs$").unwrap());

            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if let Some(captures) = PATTERN.captures(file_name) {
                        let poc_id: PocId = captures.get(2).unwrap().as_str().parse().unwrap();
                        let poc_data = PocData {
                            name: captures.get(1).unwrap().as_str().to_owned(),
                            path: entry.path(),
                        };

                        // Check no duplication
                        assert!(id_set.insert(poc_id, poc_data).is_none());
                    }
                }
            }
        }

        Ok(PocMap(id_set))
    }

    pub fn next_empty_id(&self) -> PocId {
        for id in PocId::iter_all() {
            if !self.0.contains_key(&id) {
                return id;
            }
        }

        panic!("No more PoC can be added!");
    }

    pub fn read_metadata(&self, poc_id: PocId) -> Result<Metadata> {
        let poc_data = self
            .0
            .get(&poc_id)
            .with_context(|| format!("PoC {} not found", poc_id))?;

        let content = fs::read_to_string(&poc_data.path)
            .with_context(|| format!("Cannot read {}", poc_data.name))?;

        let header_index = content.find(METADATA_HEADER);
        let footer_index = content.find(METADATA_FOOTER);

        let metadata = match (header_index, footer_index) {
            (Some(0), Some(end)) => {
                let metadata_str = &content[METADATA_HEADER.len()..end];
                toml::from_str(metadata_str)
                    .with_context(|| format!("Failed to parse metadata of {}", poc_data.name))?
            }
            _ => anyhow::bail!("PoC header was not found in {}", poc_data.name),
        };

        println!("{}", toml::to_string(&metadata).unwrap());

        Ok(metadata)
    }
}
