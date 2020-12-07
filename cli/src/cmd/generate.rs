use std::{fs, str::FromStr};

use crate::prelude::*;
use crate::{
    git::GitClient,
    poc::{Metadata, PeerMetadata, PocMap, TargetMetadata},
};

use anyhow::bail;
use askama::Template;
use chrono::prelude::*;
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
    cargo_flags: Vec<String>,
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
    original_issue_url: Option<String>,
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

    let cargo_flags = test_metadata.cargo_flags.clone();

    let os_version = cmd!("lsb_release", "-sd")
        .read()
        .context("Failed to read OS version from `lsb_release` command")?;

    let rustc_version = util::cmd_remove_cargo_envs(cmd!("rustc", "--version"))
        .dir(temp_dir.path())
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
        cargo_flags,
        poc_code,
        poc_output,
    })
}

pub fn cmd_generate(args: GenerateArgs) -> Result<()> {
    if !promptly::prompt(
        "This command will overwrite `issue_report.md` and `advisory.md` in the project directory. Is it okay?",
    )? {
        println!("No files were generated");
        return Ok(())
    }

    let (issue_report_content, mut advisory_content) = match args {
        GenerateArgs::Issue { poc_id } => {
            let issue_data = issue_data_from_id(poc_id)?;
            (
                IssueTemplate { data: issue_data }.render()?,
                String::from("Issue report does not use `advisory.md`."),
            )
        }
        GenerateArgs::Rustsec { poc_id } => {
            let poc_map = PocMap::new()?;
            let metadata = poc_map.read_metadata(poc_id)?;

            match (metadata.report.issue_url, metadata.report.issue_date) {
                (None, _) => bail!(
                    "Issue URL not found in PoC {}. \
                    Create an issue report first with `rust-poc generate issue <PoC ID>`, \
                    or use `rust-poc generate rustsec-direct <PoC ID>` \
                    if there is no issue tracker for the crate.",
                    poc_id,
                ),
                (Some(_), None) => bail!(
                    "Issue date was not found in PoC {}. \
                    Please fill in the issue_date field and try again.",
                    poc_id
                ),
                (Some(issue_url), Some(issue_date)) => {
                    let git_client = GitClient::new_with_config_file()?;
                    let issue_status = git_client.issue_status(&issue_url)?;

                    let issue_title = issue_status
                        .as_ref()
                        .map(|issue| issue.title())
                        .unwrap_or("(( Title of the original issue ))")
                        .to_owned();

                    let issue_report_content = RustsecIssueTemplate {
                        krate: metadata.target.krate.clone(),
                        original_issue_title: issue_title.clone(),
                        original_issue_url: issue_url.clone(),
                    }
                    .render()?;

                    let advisory_content = AdvisoryTemplate {
                        krate: metadata.target.krate,
                        original_issue_title: issue_title,
                        original_issue_url: Some(issue_url),
                        original_issue_date: issue_date,
                    }
                    .render()?;

                    (issue_report_content, advisory_content)
                }
            }
        }
        GenerateArgs::RustsecDirect { poc_id } => {
            let poc_map = PocMap::new()?;
            let metadata = poc_map.read_metadata(poc_id)?;
            let issue_data = issue_data_from_id(poc_id)?;

            let issue_report_content = RustsecDirectIssueTemplate { data: issue_data }.render()?;

            let local_now: DateTime<Local> = Local::now();
            let today_date: toml::value::Datetime =
                FromStr::from_str(&local_now.format("%Y-%m-%d").to_string()).unwrap();

            let advisory_content = AdvisoryTemplate {
                krate: metadata.target.krate,
                original_issue_title: String::from("((Issue Title))"),
                original_issue_url: None,
                original_issue_date: today_date,
            }
            .render()?;

            (issue_report_content, advisory_content)
        }
    };
    advisory_content.push('\n');

    fs::write(PROJECT_PATH.join("issue_report.md"), issue_report_content)
        .context("Failed to write to `issue_report.md`")?;
    fs::write(PROJECT_PATH.join("advisory.md"), advisory_content)
        .context("Failed to write to `advisory.md`")?;

    println!("Generated `issue_report.md` and `advisory.md`");

    Ok(())
}
