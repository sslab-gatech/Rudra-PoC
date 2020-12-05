use crate::poc::{Metadata, PeerMetadata, PocMap, TargetMetadata, TestMetadata};
use crate::prelude::*;

use askama::Template;
use duct::cmd;
use semver::Version;
use structopt::StructOpt;
use tempdir::TempDir;
use toml::value::Datetime;

#[derive(Debug, StructOpt)]
pub enum GenerateArgs {
    #[structopt(about = "Generates templates for bug reporting to the crate repository")]
    Issue {
        #[structopt(parse(try_from_str), help = "PoC ID (4 digits)")]
        poc_id: PocId,
    },
    #[structopt(about = "Generates templates for requesting a RustSec advisory")]
    Rustsec {
        #[structopt(parse(try_from_str), help = "PoC ID (4 digits)")]
        poc_id: PocId,
    },
    #[structopt(about = "Generates templates for direct bug reporting to RustSec advisory-db")]
    RustsecDirect {
        #[structopt(parse(try_from_str), help = "PoC ID (4 digits)")]
        poc_id: PocId,
    },
}

#[derive(Debug)]
struct IssueTemplateData {
    krate: String,
    version: Version,
    peer_dependencies: Vec<PeerMetadata>,
    os_version: String,
    rustc_version: String,
    cargo_command: String,
    poc_code: String,
    poc_output: String,
}

#[derive(Template)]
#[template(path = "generate/issue.md", escape = "none")]
struct IssueTemplate {
    data: IssueTemplateData,
}

#[derive(Template)]
#[template(path = "generate/rustsec-direct-issue.md", escape = "none")]
struct RustsecDirectIssueTemplate {
    data: IssueTemplateData,
}

#[derive(Template)]
#[template(path = "generate/rustsec-issue.md", escape = "none")]
struct RustsecIssueTemplate {
    krate: String,
    original_issue_title: String,
    original_issue_url: String,
}

#[derive(Template)]
#[template(path = "generate/advisory.md", escape = "none")]
struct AdvisoryTemplate {
    krate: String,
    original_issue_title: String,
    original_issue_url: String,
    original_issue_date: Datetime,
}

fn issue_data_from_id(poc_id: PocId) -> Result<IssueTemplateData> {
    println!("Generating the issue template from PoC...");

    let poc_map = PocMap::new()?;

    let temp_dir = TempDir::new("rudra-poc").context("Failed to create a temp directory")?;
    poc_map.prepare_poc_workspace(poc_id, temp_dir.path())?;

    let (metadata, poc_code) = poc_map.read_metadata_and_code(poc_id)?;

    let Metadata {
        target:
            TargetMetadata {
                krate,
                version,
                peer_dependencies,
            },
        test: mut test_metadata,
        report: _,
    } = metadata;

    let cargo_command = util::cargo_command_str("run", &test_metadata);

    let os_version = cmd!("lsb_release", "-sd")
        .read()
        .context("Failed to read OS version from `lsb_release` command")?;

    let rustc_version_command = util::cargo_command(
        "rustc",
        &TestMetadata {
            analyzers: Vec::new(),
            cargo_flags: vec![
                String::from("--quiet"),
                String::from("--"),
                String::from("--version"),
            ],
            cargo_toolchain: None,
        },
        temp_dir.path(),
    );
    let rustc_version = rustc_version_command
        .read()
        .context("Failed to read rustc version")?;

    let poc_build_command = util::cargo_command("build", &test_metadata, temp_dir.path());
    poc_build_command.run()?;

    test_metadata.cargo_flags.insert(0, String::from("--quiet"));
    let poc_run_command = util::cargo_command("run", &test_metadata, temp_dir.path())
        .stderr_to_stdout()
        .stdout_capture()
        .unchecked();
    let poc_run_output = poc_run_command.run()?;

    let poc_output = format!(
        "{}\n{}",
        String::from_utf8(poc_run_output.stdout).unwrap(),
        util::exit_status_string(&poc_run_output.status)
    );

    Ok(IssueTemplateData {
        krate,
        version,
        peer_dependencies,
        os_version,
        rustc_version,
        cargo_command,
        poc_code,
        poc_output,
    })
}

pub fn cmd_generate(args: GenerateArgs) -> Result<()> {
    match args {
        GenerateArgs::Issue { poc_id } => {
            let issue_data = issue_data_from_id(poc_id)?;
            println!("{}", IssueTemplate { data: issue_data }.render()?);
        }
        GenerateArgs::Rustsec { poc_id } => todo!(),
        GenerateArgs::RustsecDirect { poc_id } => {
            let issue_data = issue_data_from_id(poc_id)?;
            println!(
                "{}",
                RustsecDirectIssueTemplate { data: issue_data }.render()?
            );
        }
    }

    todo!()
}
