use std::fs;

use crate::prelude::*;
use crate::{
    git::{CratesIoClient, GitClient},
    poc::{Metadata, PocMap},
};

use anyhow::bail;
use once_cell::sync::Lazy;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum ReportArgs {
    #[structopt(about = "Reports the issue to the crate repository")]
    Issue(IssueArgs),
    #[structopt(about = "Reports the issue to RustSec advisory DB")]
    Rustsec(RustsecArgs),
}

#[derive(Debug, StructOpt)]
pub struct IssueArgs {
    #[structopt(parse(try_from_str), help = "PoC ID (4 digits)")]
    poc_id: PocId,
}

#[derive(Debug, StructOpt)]
pub struct RustsecArgs {
    #[structopt(parse(try_from_str), help = "PoC ID (4 digits)")]
    poc_id: PocId,
}

struct IssueData {
    title: String,
    body: String,
}

fn parse_issue() -> Result<IssueData> {
    let issue_report_content = fs::read_to_string(PROJECT_PATH.join("issue_report.md"))
        .context("Failed to read `issue_report.md`")?;

    let mut lines = issue_report_content.lines();
    let first_line = lines.next().unwrap();
    if !first_line.starts_with("# ") {
        bail!("The first line of the issue report does not start with `# `");
    }
    let title = first_line[2..].trim().to_owned();

    let body = issue_report_content[first_line.len() + 1..]
        .trim()
        .to_owned();

    Ok(IssueData { title, body })
}

/// Validate the advisory content and returns it
fn validate_advisory(metadata: &Metadata) -> Result<String> {
    let issue_report_content = fs::read_to_string(PROJECT_PATH.join("advisory.md"))
        .context("Failed to read `advisory.md`")?;

    let lines: Vec<_> = issue_report_content.lines().collect();
    let start_index = lines.iter().position(|&s| s == "```toml");
    let end_index = lines.iter().position(|&s| s == "```");

    match (start_index, end_index) {
        (Some(0), Some(end_index)) => {
            // Ensure there is no comment in the metadata
            if lines[1..end_index].iter().any(|s| s.starts_with('#')) {
                bail!("Please remove all comments in `advisory.md` before reporting");
            }

            // Ensure that the package name in advisory matches the crate name of PoC
            static PACKAGE_REGEX: Lazy<Regex> =
                Lazy::new(|| Regex::new(r#"package\s*=\s*"(?P<crate>[A-Za-z0-9_-]+)""#).unwrap());
            let krate = lines[1..end_index].iter().find_map(|s| {
                if let Some(captures) = PACKAGE_REGEX.captures(s) {
                    let krate = captures.name("crate").unwrap().as_str();
                    Some(krate)
                } else {
                    None
                }
            });
            if krate != Some(&metadata.target.krate) {
                bail!(
                    "Crate name in `advisory.md` ({}) does not match the crate name of the PoC ({})",
                    krate.unwrap_or("<not found>"),
                    &metadata.target.krate
                );
            }
        }
        _ => {
            bail!("Metadata malformed in `advisory.md`");
        }
    }

    Ok(issue_report_content)
}

fn report_to_crate_repository(args: IssueArgs) -> Result<()> {
    let poc_map = PocMap::new()?;
    let mut metadata = poc_map.read_metadata(args.poc_id)?;

    if metadata.report.issue_url.is_some() {
        bail!("This PoC was already reported to the crate repository!");
    }

    let issue_data = parse_issue()?;
    println!("[[ Issue Title ]]\n{}", issue_data.title);
    println!("[[ Issue Body ]]\n{}", issue_data.body);

    if !promptly::prompt(
        "Do you want to submit an issue to the crate repository with the content above?",
    )? {
        println!("The issue was not submitted");
        return Ok(());
    }

    let crate_name = &metadata.target.krate;
    println!(
        "Fetching the repository URL for `{}` from crates.io...",
        crate_name
    );
    let crates_io_client = CratesIoClient::new()?;
    let repository_url = crates_io_client.repository_url(crate_name)?;

    if let Some(url) = &repository_url {
        static GITHUB_REPOSITORY_URL: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^https://github.com/(?P<owner>[^/]+)/(?P<repo>[^/]+)").unwrap()
        });

        if let Some(captures) = GITHUB_REPOSITORY_URL.captures(url) {
            let owner = captures.name("owner").unwrap().as_str();
            let repo = captures.name("repo").unwrap().as_str();

            println!("Reporting to GitHub repository {}/{}...", owner, repo);
            let git_client = GitClient::new_with_config_file()?;
            let repo_url =
                git_client.create_github_issue(owner, repo, &issue_data.title, &issue_data.body)?;

            metadata.report.issue_url = Some(repo_url);
            metadata.report.issue_date = Some(util::today_toml_date());
            poc_map.write_metadata(args.poc_id, metadata)?;

            crate::cmd::update::update_readme()?;

            return Ok(());
        }
    }

    // Gracefully handling the fallback case
    let url = match &repository_url {
        Some(url) => url,
        None => "<not found>",
    };

    println!(
        "Automatic issue submission is not supported for URL {}",
        url
    );

    Ok(())
}

fn report_to_rustsec(args: RustsecArgs) -> Result<()> {
    let poc_map = PocMap::new()?;
    let mut metadata = poc_map.read_metadata(args.poc_id)?;

    if metadata.report.rustsec_url.is_some() {
        bail!("This PoC was already reported to RustSec advisory DB!");
    }

    let issue_data = parse_issue()?;
    let advisory_content = validate_advisory(&metadata)?;

    println!("[[ PR Title ]]\n{}", issue_data.title);
    println!("[[ PR Body ]]\n{}", issue_data.body);
    println!("[[ Advisory Body ]]\n{}", advisory_content);

    if !promptly::prompt(
        "Do you want to submit a PR to RustSec advisory DB with the content above?",
    )? {
        println!("The PR was not submitted");
        return Ok(());
    }

    // Create commit and PR
    let git_client = GitClient::new_with_config_file()?;

    let crate_name = &metadata.target.krate;
    let branch_name = format!("{}-{}", args.poc_id, crate_name);

    git_client.commit_and_push(
        &branch_name,
        format!("crates/{}/RUSTSEC-0000-0000.md", crate_name),
        &advisory_content,
    )?;

    let pr_url = git_client.create_rustsec_pr(&branch_name, &issue_data.title, &issue_data.body)?;
    println!("Successfully submitted a PR: {}", pr_url);

    metadata.report.rustsec_url = Some(pr_url);
    poc_map.write_metadata(args.poc_id, metadata)?;

    crate::cmd::update::update_readme()?;

    Ok(())
}

pub fn cmd_report(args: ReportArgs) -> Result<()> {
    match args {
        ReportArgs::Issue(args) => report_to_crate_repository(args),
        ReportArgs::Rustsec(args) => report_to_rustsec(args),
    }
}
