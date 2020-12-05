use std::{collections::HashMap, fs, os::unix::fs::symlink, path::Path, path::PathBuf};

use crate::prelude::*;

use askama::Template;
use once_cell::sync::Lazy;
use semver::Version;
use serde::{Deserialize, Serialize};
use toml::value::Datetime;

static METADATA_HEADER: &str = "/*!\n```rudra-poc\n";
static METADATA_FOOTER: &str = "```\n!*/\n";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub target: TargetMetadata,
    pub test: TestMetadata,
    pub report: ReportMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetMetadata {
    #[serde(rename = "crate")]
    pub krate: String,
    pub version: Version,
    #[serde(default, rename = "peer")]
    pub peer_dependencies: Vec<PeerMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerMetadata {
    #[serde(rename = "crate")]
    pub krate: String,
    pub version: Version,
    #[serde(default)]
    pub features: Vec<String>,
}

impl std::fmt::Display for PeerMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {{ version = \"{}\"", self.krate, self.version)?;
        if !self.features.is_empty() {
            let mut iter = self.features.iter();
            write!(f, ", features = [\"{}\"", iter.next().unwrap())?;
            for feature in iter {
                write!(f, ", \"{}\"", feature)?;
            }
            write!(f, "]")?;
        }
        write!(f, " }}")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Analyzer {
    #[serde(rename = "manual")]
    Manual,
    UnsafeDestructor,
    SendSyncChecker,
}

impl std::fmt::Display for Analyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Analyzer::Manual => "manual",
            Analyzer::UnsafeDestructor => "UnsafeDestructor",
            Analyzer::SendSyncChecker => "SendSyncChecker",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMetadata {
    pub analyzers: Vec<Analyzer>,
    #[serde(default)]
    pub cargo_flags: Vec<String>,
    pub cargo_toolchain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub issue_url: Option<String>,
    pub issue_date: Option<Datetime>,
    pub rustsec_url: Option<String>,
    pub rustsec_id: Option<String>,
}

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
            static POC_FILENAME_REGEX: Lazy<Regex> = Lazy::new(|| {
                Regex::new(r"^(?P<file_stem>(?P<poc_id>\d{4})-[A-Za-z0-9\-_]+)\.rs$").unwrap()
            });

            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if let Some(captures) = POC_FILENAME_REGEX.captures(file_name) {
                        let poc_id: PocId =
                            captures.name("poc_id").unwrap().as_str().parse().unwrap();
                        let poc_data = PocData {
                            name: captures.name("file_stem").unwrap().as_str().to_owned(),
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

    fn get(&self, poc_id: PocId) -> Result<&PocData> {
        self.0
            .get(&poc_id)
            .with_context(|| format!("PoC {} not found", poc_id))
    }

    pub fn read_metadata(&self, poc_id: PocId) -> Result<Metadata> {
        let poc_data = self.get(poc_id)?;

        let content = fs::read_to_string(&poc_data.path)
            .with_context(|| format!("Cannot read {}", poc_data.name))?;

        let header_index = content.find(METADATA_HEADER);
        let footer_index = content.find(METADATA_FOOTER);

        match (header_index, footer_index) {
            (Some(0), Some(end)) => {
                let metadata_str = &content[METADATA_HEADER.len()..end];
                let metadata = toml::from_str(metadata_str)
                    .with_context(|| format!("Failed to parse metadata of {}", poc_data.name))?;

                Ok(metadata)
            }
            _ => anyhow::bail!("PoC header was not found in {}", poc_data.name),
        }
    }

    pub fn read_metadata_and_code(&self, poc_id: PocId) -> Result<(Metadata, String)> {
        let poc_data = self.get(poc_id)?;

        let content = fs::read_to_string(&poc_data.path)
            .with_context(|| format!("Cannot read {}", poc_data.name))?;

        let header_index = content.find(METADATA_HEADER);
        let footer_index = content.find(METADATA_FOOTER);

        match (header_index, footer_index) {
            (Some(0), Some(end)) => {
                let metadata_str = &content[METADATA_HEADER.len()..end];
                let metadata = toml::from_str(metadata_str)
                    .with_context(|| format!("Failed to parse metadata of {}", poc_data.name))?;

                let poc_code = content[end + METADATA_FOOTER.len()..].to_owned();

                Ok((metadata, poc_code))
            }
            _ => anyhow::bail!("PoC header was not found in {}", poc_data.name),
        }
    }

    pub fn prepare_poc_workspace(
        &self,
        poc_id: PocId,
        workspace_dir: impl AsRef<Path>,
    ) -> Result<()> {
        #[derive(Template)]
        #[template(path = "workspace/Cargo.toml", escape = "none")]
        struct CargoTomlTemplate {
            poc_id: PocId,
            metadata: Metadata,
        }

        #[derive(Template)]
        #[template(path = "workspace/build.rs", escape = "none")]
        struct BuildRsTemplate {
            dependency_path: PathBuf,
        }

        let workspace_dir = workspace_dir.as_ref();
        if workspace_dir.exists() {
            fs::remove_dir_all(workspace_dir)?;
        }

        let src_dir = workspace_dir.join("src");
        fs::create_dir_all(src_dir)?;

        // Cargo.toml
        let template = CargoTomlTemplate {
            poc_id,
            metadata: self.read_metadata(poc_id)?,
        };
        fs::write(workspace_dir.join("Cargo.toml"), template.render()?)?;

        // build.rs
        let template = BuildRsTemplate {
            dependency_path: PROJECT_PATH.join("dependencies"),
        };
        fs::write(workspace_dir.join("build.rs"), template.render()?)?;

        // src/{poc_name}.rs
        let poc_data = self.get(poc_id)?;
        symlink(&poc_data.path, workspace_dir.join("src/main.rs"))?;

        // src/boilerplate.rs
        symlink(
            PROJECT_PATH.join("poc/boilerplate.rs"),
            workspace_dir.join("src/boilerplate.rs"),
        )?;

        Ok(())
    }

    pub fn iter_ids(&self) -> impl Iterator<Item = PocId> {
        let mut vec: Vec<_> = self.0.keys().map(|key| *key).collect();
        vec.sort();
        vec.into_iter()
    }
}
