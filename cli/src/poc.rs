use std::{
    collections::HashMap, fs, os::unix::fs::symlink, path::Path, path::PathBuf, process::Command,
    process::ExitStatus,
};

use crate::prelude::*;

use askama::Template;
use once_cell::sync::Lazy;
use semver::Version;
use serde::{Deserialize, Serialize};
use toml::value::Datetime;

static METADATA_HEADER: &str = "/*!\n```rudra-poc\n";
static METADATA_FOOTER: &str = "```\n";

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
    #[serde(default)]
    pub peer: Vec<PeerMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerMetadata {
    #[serde(rename = "crate")]
    pub krate: String,
    pub version: Version,
    #[serde(default)]
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Analyzer {
    #[serde(rename = "manual")]
    Manual,
    UnsafeDestructor,
    SendSyncChecker,
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

pub fn cargo_command(subcommand: &str, metadata: &Metadata) -> Command {
    let mut command = Command::new("cargo");

    if let Some(toolchain) = &metadata.test.cargo_toolchain {
        command.arg(format!("+{}", toolchain));
    }

    command.arg(subcommand);
    command.args(&metadata.test.cargo_flags);

    command
}

// https://man7.org/linux/man-pages/man7/signal.7.html
pub fn signal_name(signal: i32) -> &'static str {
    match signal {
        1 => "SIGHUP",
        2 => "SIGINT",
        3 => "SIGQUIT",
        4 => "SIGILL",
        5 => "SIGTRAP",
        6 => "SIGABRT",
        7 => "SIGBUS",
        8 => "SIGFPE",
        9 => "SIGKILL",
        10 => "SIGUSR1",
        11 => "SIGSEGV",
        12 => "SIGUSR2",
        13 => "SIGPIPE",
        14 => "SIGALRM",
        15 => "SIGTERM",
        16 => "SIGTKFLT",
        17 => "SIGCHLD",
        18 => "SIGCONT",
        19 => "SIGSTOP",
        20 => "SIGTSTP",
        21 => "SIGTTIN",
        22 => "SIGTTOU",
        23 => "SIGURG",
        24 => "SIGXCPU",
        25 => "SIGXFSZ",
        26 => "SIGVTARLM",
        27 => "SIGPROF",
        28 => "SIGWINCH",
        29 => "SIGIO",
        30 => "SIGPWR",
        31 => "SIGSYS",
        _ => "Unknown",
    }
}

pub fn exit_status_string(exit_status: &ExitStatus) -> String {
    use std::os::unix::process::ExitStatusExt;

    if let Some(signal) = exit_status.signal() {
        format!(
            "Terminated with signal {} ({})",
            signal,
            signal_name(signal)
        )
    } else if let Some(return_code) = exit_status.code() {
        format!("Return code {}", return_code)
    } else {
        String::from("Unknown return status")
    }
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

        let metadata = match (header_index, footer_index) {
            (Some(0), Some(end)) => {
                let metadata_str = &content[METADATA_HEADER.len()..end];
                toml::from_str(metadata_str)
                    .with_context(|| format!("Failed to parse metadata of {}", poc_data.name))?
            }
            _ => anyhow::bail!("PoC header was not found in {}", poc_data.name),
        };

        Ok(metadata)
    }

    pub fn prepare_poc_workspace(
        &self,
        poc_id: PocId,
        workspace_dir: impl AsRef<Path>,
    ) -> Result<()> {
        #[derive(Template)]
        #[template(path = "poc-debug/Cargo.toml", escape = "none")]
        struct CargoTomlTemplate {
            poc_id: PocId,
            metadata: Metadata,
        }

        #[derive(Template)]
        #[template(path = "poc-debug/build.rs", escape = "none")]
        struct BuildRsTemplate {
            dependency_path: PathBuf,
        }

        mod filters {
            pub fn feature_filter(features: &Vec<String>) -> askama::Result<String> {
                Ok(features
                    .iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<_>>()
                    .join(", "))
            }
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
}
