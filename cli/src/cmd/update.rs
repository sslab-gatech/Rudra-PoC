use std::fs;

use crate::git::GitClient;
use crate::poc::PocMap;
use crate::prelude::*;

use askama::Template;
use once_cell::sync::Lazy;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct UpdateArgs {}

enum LinkStyle {
    Image { alt_text: String, image_url: String },
    Text(String),
    Plaintext,
}

struct MdLink {
    style: LinkStyle,
    url: String,
}

impl MdLink {
    pub fn image(alt_text: impl ToString, image_url: impl ToString, url: impl ToString) -> Self {
        MdLink {
            style: LinkStyle::Image {
                alt_text: alt_text.to_string(),
                image_url: image_url.to_string(),
            },
            url: url.to_string(),
        }
    }

    pub fn text(text: impl ToString, url: impl ToString) -> Self {
        MdLink {
            style: LinkStyle::Text(text.to_string()),
            url: url.to_string(),
        }
    }

    pub fn plaintext(url: impl ToString) -> Self {
        MdLink {
            style: LinkStyle::Plaintext,
            url: url.to_string(),
        }
    }
}

impl std::fmt::Display for MdLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.style {
            LinkStyle::Image {
                alt_text,
                image_url,
            } => write!(f, "[![{}]({})]({})", alt_text, image_url, self.url),
            LinkStyle::Text(text) => write!(f, "[{}]({})", text, self.url),
            LinkStyle::Plaintext => write!(f, "{}", self.url),
        }
    }
}

static GITHUB_ISSUE_PR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https://github.com/(?P<user>[^/]+)/(?P<repo>[^/]+)/(?P<issue_or_pr>issues|pull)/(?P<number>\d+)$").unwrap()
});

static GITLAB_ISSUE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https://gitlab.com/(?P<user>[^/]+)/(?P<repo>[^/]+)/-/issues/(?P<number>\d+)$")
        .unwrap()
});

static ADVISORY_DB_PR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https://github.com/RustSec/advisory-db/pull/(?P<pr_number>\d+)$").unwrap()
});

fn issue_link(url: &str) -> MdLink {
    if let Some(captures) = GITHUB_ISSUE_PR_REGEX.captures(url) {
        // GitHub
        let user = captures.name("user").unwrap().as_str();
        let repo = captures.name("repo").unwrap().as_str();
        let issue_or_pr = captures.name("issue_or_pr").unwrap().as_str();
        let number = captures.name("number").unwrap().as_str();

        let label = format!("{}%2f{}%23{}", user, repo, number);
        let image_url = format!(
            "https://img.shields.io/github/{}/detail/state/{}/{}/{}?label={}&logo=GitHub&cacheSeconds=3600&style=flat-square",
            if issue_or_pr == "pull" {
                "pulls"
            } else {
                issue_or_pr
            },
            user,
            repo,
            number,
            label
        );
        MdLink::image("GitHub issue or PR", image_url, url)
    } else if let Some(captures) = GITLAB_ISSUE_REGEX.captures(url) {
        // GitLab
        let user = captures.name("user").unwrap().as_str();
        let repo = captures.name("repo").unwrap().as_str();
        let number = captures.name("number").unwrap().as_str();

        let label = format!(
            "{}%2f{}%23{}",
            user.replace("-", "--"),
            repo.replace("-", "--"),
            number
        );
        let image_url = format!(
            "https://img.shields.io/badge/{}-grey?logo=GitLab&style=flat-square",
            label
        );

        MdLink::image("GitLab issue", image_url, url)
    } else {
        MdLink::plaintext(url)
    }
}

fn rustsec_id_link(rustsec_id: &str) -> MdLink {
    MdLink::image(
        rustsec_id,
        format!(
            "https://img.shields.io/badge/RUSTSEC-{}--{}-blue?style=flat-square",
            &rustsec_id[8..12],
            &rustsec_id[13..17]
        ),
        format!("https://rustsec.org/advisories/{}.html", rustsec_id),
    )
}

fn rustsec_url_link(rustsec_url: &str) -> MdLink {
    match ADVISORY_DB_PR_REGEX.captures(rustsec_url) {
        Some(captures) => {
            let pr_number = captures.name("pr_number").unwrap().as_str();
            MdLink::image(
                "GitHub pull request detail",
                format!(
                    "https://img.shields.io/github/pulls/detail/state/RustSec/advisory-db/{}?style=flat-square",
                    pr_number
                ),
                rustsec_url,
            )
        }
        None => MdLink::plaintext(rustsec_url),
    }
}

#[derive(Template)]
#[template(path = "README.md")]
struct ReadmeTemplate {
    lines: Vec<ReadmeTemplateLine>,
}

impl ReadmeTemplate {
    pub fn new() -> Self {
        ReadmeTemplate { lines: Vec::new() }
    }

    pub fn push(&mut self, line: ReadmeTemplateLine) {
        self.lines.push(line);
    }
}

struct ReadmeTemplateLine {
    poc_id: PocId,
    krate: MdLink,
    analyzers: Vec<String>,
    issue_url: Option<MdLink>,
    rustsec_link: Option<MdLink>,
}

mod filters {
    pub fn unordered_list(vec: &Vec<String>) -> askama::Result<String> {
        let mut s = String::new();

        let mut iter = vec.iter();
        s.push_str("- ");
        s.push_str(iter.next().unwrap());

        for analyzer_name in iter {
            s.push_str("<br>- ");
            s.push_str(analyzer_name);
        }

        Ok(s)
    }

    pub fn unwrap_or(
        s: &Option<impl ToString>,
        default_value: &'static str,
    ) -> askama::Result<String> {
        match s.as_ref() {
            Some(s) => Ok(s.to_string()),
            None => Ok(String::from(default_value)),
        }
    }
}

pub fn update_readme() -> Result<()> {
    let poc_map = PocMap::new()?;

    let mut readme_template = ReadmeTemplate::new();
    for poc_id in poc_map.iter_ids() {
        let metadata = poc_map.read_metadata(poc_id)?;

        // ![Crates.io (recent)](https://img.shields.io/crates/dr/rustc-serialize)
        let krate_name = &metadata.target.krate;
        let krate = MdLink::text(
            krate_name,
            format!("https://crates.io/crates/{}", krate_name),
        );

        let analyzers: Vec<_> = metadata
            .test
            .analyzers
            .iter()
            .map(|analyzer| analyzer.to_string())
            .collect();
        let issue_url = metadata.report.issue_url.as_ref().map(|s| issue_link(s));
        let rustsec_link = match (
            metadata.report.rustsec_id.as_ref(),
            metadata.report.rustsec_url.as_ref(),
        ) {
            (Some(rustsec_id), Some(_)) => Some(rustsec_id_link(rustsec_id)),
            (Some(_), None) => {
                anyhow::bail!("Invalid PoC metadata in {}: Contains RUSTSEC ID without RUSTSEC URL")
            }
            (None, Some(rustsec_url)) => Some(rustsec_url_link(rustsec_url)),
            _ => None,
        };

        let line = ReadmeTemplateLine {
            poc_id,
            krate,
            analyzers,
            issue_url,
            rustsec_link,
        };

        readme_template.push(line);
    }

    let readme_content = readme_template.render()?;
    fs::write(PROJECT_PATH.join("README.md"), readme_content)?;

    println!("Successfully updated README.md");

    Ok(())
}

pub fn cmd_update(_args: UpdateArgs) -> Result<()> {
    // TODO: detect and update RUSTSEC ID from git repository
    let git_client = GitClient::new_with_config_file()?;
    let _repository = git_client.prepare_rustsec_local()?;

    update_readme()?;

    Ok(())
}
