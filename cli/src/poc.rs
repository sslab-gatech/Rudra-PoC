use std::{collections::HashMap, fs, os::unix::fs::symlink, path::Path, path::PathBuf};

use crate::prelude::*;

use anyhow::bail;
use askama::Template;
use once_cell::sync::Lazy;
use semver::Version;
use serde::{Deserialize, Serialize};
use toml::value::Datetime;

static METADATA_HEADER: &str = "/*!\n```rudra-poc\n";
static METADATA_FOOTER: &str = "```\n!*/\n";

fn empty_test_metadata(metadata: &TestMetadata) -> bool {
    metadata.cargo_flags.is_empty() && metadata.cargo_toolchain.is_none()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub target: TargetMetadata,
    #[serde(default, skip_serializing_if = "empty_test_metadata")]
    pub test: TestMetadata,
    pub report: ReportMetadata,
    pub bugs: Vec<BugMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetMetadata {
    #[serde(rename = "crate")]
    pub krate: String,
    pub version: Version,
    #[serde(default, rename = "peer", skip_serializing_if = "Vec::is_empty")]
    pub peer_dependencies: Vec<PeerMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerMetadata {
    #[serde(rename = "crate")]
    pub krate: String,
    pub version: Version,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    Manual,
    UnsafeDestructor,
    SendSyncVariance,
    UnsafeDataflow,
}

impl std::fmt::Display for Analyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Analyzer::Manual => "Manual",
            Analyzer::UnsafeDestructor => "UnsafeDestructor",
            Analyzer::SendSyncVariance => "SendSyncVariance",
            Analyzer::UnsafeDataflow => "UnsafeDataflow",
        };
        write!(f, "{}", name)
    }
}

impl Analyzer {
    pub fn initial(&self) -> &'static str {
        match self {
            Analyzer::Manual => "M",
            Analyzer::UnsafeDestructor => "D",
            Analyzer::SendSyncVariance => "SV",
            Analyzer::UnsafeDataflow => "UD",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BugClass {
    SendSyncVariance,
    UninitExposure,
    InconsistencyAmplification,
    PanicSafety,
    Other,
}

impl std::fmt::Display for BugClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            BugClass::SendSyncVariance => "SendSyncVariance",
            BugClass::UninitExposure => "UninitExposure",
            BugClass::InconsistencyAmplification => "InconsistencyAmplification",
            BugClass::PanicSafety => "PanicSafety",
            BugClass::Other => "Other",
        };
        write!(f, "{}", name)
    }
}

impl BugClass {
    pub fn initial(&self) -> &'static str {
        match self {
            BugClass::SendSyncVariance => "SV",
            BugClass::UninitExposure => "UE",
            BugClass::InconsistencyAmplification => "IA",
            BugClass::PanicSafety => "PS",
            BugClass::Other => "O",
        }
    }
}

fn usize_one() -> usize {
    1
}

fn usize_is_one(num: &usize) -> bool {
    *num == 1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugMetadata {
    analyzer: Analyzer,
    guide: Option<Analyzer>,
    bug_class: BugClass,
    #[serde(default = "usize_one", skip_serializing_if = "usize_is_one")]
    bug_count: usize,
}

impl BugMetadata {
    pub fn initial(&self) -> String {
        format!(
            "{}{}-{} {}",
            self.analyzer.initial(),
            if self.guide.is_some() { "*" } else { "" },
            self.bug_class.initial(),
            self.bug_count,
        )
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TestMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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

    pub fn get_path_to_poc_code(&self, poc_id: PocId) -> Result<&PathBuf> {
        let poc_data = self.get(poc_id)?;
        Ok(&poc_data.path)
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
            _ => bail!("PoC header was not found in {}", poc_data.name),
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

                let poc_code = content[end + METADATA_FOOTER.len()..].trim().to_owned();

                Ok((metadata, poc_code))
            }
            _ => bail!("PoC header was not found in {}", poc_data.name),
        }
    }

    pub fn write_metadata(self, poc_id: PocId, metadata: Metadata) -> Result<()> {
        println!("Updating PoC metadata for {}...", poc_id);

        let poc_data = self.get(poc_id)?;

        let content = fs::read_to_string(&poc_data.path)
            .with_context(|| format!("Cannot read {}", poc_data.name))?;

        let header_index = content.find(METADATA_HEADER);
        let footer_index = content.find(METADATA_FOOTER);

        match (header_index, footer_index) {
            (Some(0), Some(end)) => {
                let metadata_header = format!(
                    "{}{}{}",
                    METADATA_HEADER,
                    toml::to_string(&metadata)?,
                    METADATA_FOOTER
                );
                let poc_code = content[end + METADATA_FOOTER.len()..].trim();

                fs::write(&poc_data.path, format!("{}{}\n", metadata_header, poc_code))
                    .with_context(|| {
                        format!("Failed to write to `{}`", &poc_data.path.display())
                    })?;

                Ok(())
            }
            _ => bail!("PoC header was not found in {}", poc_data.name),
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

        let metadata = self.read_metadata(poc_id)?;

        // rust-toolchain
        fs::write(
            workspace_dir.join("rust-toolchain"),
            match metadata.test.cargo_toolchain.as_ref() {
                Some(s) => s.as_ref(),
                None => "stable",
            },
        )?;

        // Cargo.toml
        let template = CargoTomlTemplate { poc_id, metadata };
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
