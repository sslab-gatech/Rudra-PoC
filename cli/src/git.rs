use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::prelude::*;

use anyhow::bail;
use duct::cmd;
use once_cell::sync::Lazy;
use reqwest::{blocking::Client, header, StatusCode};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    name: String,
    email: String,
    github_id: String,
    token: String,
    rustsec_fork_url: String,
}

impl AuthConfig {
    pub fn from_config_file() -> Result<Self> {
        let config_str = fs::read_to_string(PROJECT_PATH.join("config.toml"))
            .context("config.toml not found in the project directory")?;

        Ok(toml::from_str(&config_str)?)
    }
}

#[derive(Deserialize)]
pub struct GitHubIssue {
    id: u64,
    number: u64,
    state: String,
    title: String,
    body: String,
}

impl GitHubIssue {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> &str {
        &self.body
    }
}

pub struct CratesIoClient {
    client: Client,
}

impl CratesIoClient {
    pub fn new() -> Result<Self> {
        let mut crates_io_headers = header::HeaderMap::new();
        crates_io_headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str("Rudra project (sslab@cc.gatech.edu)").unwrap(),
        );
        let client = Client::builder()
            .default_headers(crates_io_headers)
            .build()?;

        Ok(CratesIoClient { client })
    }

    pub fn repository_url(&self, crate_name: &str) -> Result<Option<String>> {
        #[derive(Deserialize)]
        struct CrateMetadata {
            #[serde(rename = "crate")]
            krate: CrateMetadataInner,
        }

        #[derive(Deserialize)]
        struct CrateMetadataInner {
            repository: Option<String>,
        }

        let response = self
            .client
            .get(&format!("https://crates.io/api/v1/crates/{}", crate_name))
            .send()?;

        if response.status() != StatusCode::OK {
            bail!(
                "Failed to read crate metadata from crates.io - status code {}",
                response.status().as_u16()
            );
        }

        let res_body: CrateMetadata = response.json()?;
        Ok(res_body.krate.repository)
    }
}

pub struct GitClient {
    auth_config: AuthConfig,
    github_client: Client,
}

static GITHUB_ISSUE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^https://github.com/(?P<owner>[^/]+)/(?P<repo>[^/]+)/issues/(?P<issue_number>\d+)$",
    )
    .unwrap()
});

static GITHUB_PULL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https://github.com/(?P<owner>[^/]+)/(?P<repo>[^/]+)/pull/(?P<pull_number>\d+)$")
        .unwrap()
});

impl GitClient {
    pub fn new(auth_config: AuthConfig) -> Result<Self> {
        // For GitHub API access
        let mut github_headers = header::HeaderMap::new();
        github_headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/vnd.github.v3+json"),
        );
        github_headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&auth_config.github_id).unwrap(),
        );
        github_headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("token {}", auth_config.token)).unwrap(),
        );
        let github_client = Client::builder().default_headers(github_headers).build()?;

        Ok(GitClient {
            auth_config,
            github_client,
        })
    }

    pub fn new_with_config_file() -> Result<Self> {
        Self::new(AuthConfig::from_config_file()?)
    }

    fn github_remote_callbacks(&self) -> git2::RemoteCallbacks<'_> {
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |_url, _username, _allowed_types| {
            git2::Cred::userpass_plaintext(&self.auth_config.github_id, &self.auth_config.token)
        });

        callbacks
    }

    pub fn rustsec_local_path() -> PathBuf {
        PROJECT_PATH.join("advisory-db")
    }

    pub fn prepare_rustsec_local(&self) -> Result<git2::Repository> {
        let repo_path = GitClient::rustsec_local_path();
        if repo_path.exists() {
            println!("Opening git repository at `{}`...", repo_path.display());

            // Try to open the existing repository
            Ok(git2::Repository::open(&repo_path).with_context(|| {
                format!("Falied to open {} as git repository", repo_path.display())
            })?)
        } else {
            println!("Cloning rustsec fork at `{}`...", repo_path.display());

            // Try to clone to the specified location
            let mut fetch_opts = git2::FetchOptions::new();
            fetch_opts.remote_callbacks(self.github_remote_callbacks());

            let mut builder = git2::build::RepoBuilder::new();
            builder.fetch_options(fetch_opts);

            let repository = builder
                .clone(&self.auth_config.rustsec_fork_url, &repo_path)
                .with_context(|| {
                    format!(
                        "Failed to clone {} to {}",
                        &self.auth_config.rustsec_fork_url,
                        repo_path.display()
                    )
                })?;

            // Add RustSec repository to remote named "rustsec"
            repository.remote("rustsec", "https://github.com/RustSec/advisory-db.git")?;

            Ok(repository)
        }
    }

    pub fn issue_status(&self, url: &str) -> Result<Option<GitHubIssue>> {
        if let Some(captures) = GITHUB_ISSUE_REGEX.captures(url) {
            // GitHub issues
            let owner = captures.name("owner").unwrap().as_str();
            let repo = captures.name("repo").unwrap().as_str();
            let issue_number = captures.name("issue_number").unwrap().as_str();

            let response: GitHubIssue = self
                .github_client
                .get(&format!(
                    "https://api.github.com/repos/{}/{}/issues/{}",
                    owner, repo, issue_number
                ))
                .send()?
                .json()
                .with_context(|| format!("Failed to read fetch data for {}", url))?;

            Ok(Some(response))
        } else if let Some(captures) = GITHUB_PULL_REGEX.captures(url) {
            // GitHub pulls
            let owner = captures.name("owner").unwrap().as_str();
            let repo = captures.name("repo").unwrap().as_str();
            let issue_number = captures.name("issue_number").unwrap().as_str();

            let response: GitHubIssue = self
                .github_client
                .get(&format!(
                    "https://api.github.com/repos/{}/{}/pulls/{}",
                    owner, repo, issue_number
                ))
                .send()?
                .json()
                .with_context(|| format!("Failed to read fetch data for {}", url))?;

            Ok(Some(response))
        } else {
            Ok(None)
        }
    }

    pub fn push_branch(&self, branch_name: &str) -> Result<()> {
        println!("Pushing branch {}...", branch_name);

        let repo_path = GitClient::rustsec_local_path();
        let repository = git2::Repository::open(&repo_path)
            .with_context(|| format!("Failed to open {} as git repository", repo_path.display()))?;

        let mut remote = repository
            .find_remote("origin")
            .context("Can't find remote origin")?;

        let refspecs: &[String] = &[format!(
            "refs/heads/{}:refs/heads/{}",
            branch_name, branch_name
        )];
        remote
            .push(
                refspecs,
                Some(git2::PushOptions::new().remote_callbacks(self.github_remote_callbacks())),
            )
            .with_context(|| format!("Failed to push {} branch", branch_name))?;

        Ok(())
    }

    pub fn commit_and_push(
        &self,
        branch_name: &str,
        path: impl AsRef<Path>,
        content: &str,
    ) -> Result<()> {
        // Prepare the repository, but unlock it immediately
        // We will use git commandline when we don't need a credential
        drop(self.prepare_rustsec_local()?);

        let repo_path = GitClient::rustsec_local_path();

        // Update master
        println!("Updating master branch...");
        util::cmd_run_silent(cmd!("git", "checkout", "master"), &repo_path).run()?;
        util::cmd_run_silent(cmd!("git", "fetch", "rustsec"), &repo_path).run()?;
        util::cmd_run_silent(
            cmd!("git", "merge", "rustsec/master", "--ff-only"),
            &repo_path,
        )
        .run()?;

        self.push_branch("master")?;

        // Create and push the PoC branch
        println!("Creating branch {}...", branch_name);
        util::cmd_run_silent(cmd!("git", "checkout", "-b", &branch_name), &repo_path)
            .run()
            .context("Failed to checkout, maybe the branch already exists?")?;

        let advisory_path = repo_path.join(path.as_ref());
        let parent_dir = advisory_path.parent().unwrap();
        fs::create_dir_all(parent_dir)
            .with_context(|| format!("Failed to create dir {}", parent_dir.display()))?;
        fs::write(&advisory_path, content)
            .with_context(|| format!("Failed to write to {}", advisory_path.display()))?;

        util::cmd_run_silent(cmd!("git", "add", "-A"), &repo_path).run()?;
        util::cmd_run_silent(
            cmd!(
                "git",
                "commit",
                "-m",
                format!("Report {} to RustSec", branch_name)
            ),
            &repo_path,
        )
        .run()?;

        self.push_branch(branch_name)?;

        util::cmd_run_silent(
            cmd!("git", "branch", "-u", &format!("origin/{}", branch_name)),
            &repo_path,
        )
        .run()
        .with_context(|| format!("Failed to set upstream for branch {}", branch_name))?;

        Ok(())
    }

    pub fn create_rustsec_pr(&self, branch_name: &str, title: &str, body: &str) -> Result<String> {
        let head_str = format!("{}:{}", self.auth_config.github_id, branch_name);
        let mut req_body = HashMap::new();
        req_body.insert("title", title);
        req_body.insert("head", &head_str);
        req_body.insert("base", "master");
        req_body.insert("body", body);

        #[derive(Deserialize)]
        struct PrCreationResponse {
            // Ignore all unnecessary fields
            html_url: String,
        }

        let response = self
            .github_client
            .post("https://api.github.com/repos/RustSec/advisory-db/pulls")
            .json(&req_body)
            .send()?;

        if response.status() != StatusCode::CREATED {
            bail!(
                "Failed to create a RustSec pull request - status code {}",
                response.status().as_u16()
            );
        }

        let res_body: PrCreationResponse = response.json()?;
        Ok(res_body.html_url)
    }

    pub fn create_github_issue(
        &self,
        owner: &str,
        repo: &str,
        title: &str,
        body: &str,
    ) -> Result<String> {
        let mut req_body = HashMap::new();
        req_body.insert("title", title);
        req_body.insert("body", body);

        let response = self
            .github_client
            .post(&format!(
                "https://api.github.com/repos/{}/{}/issues",
                owner, repo
            ))
            .json(&req_body)
            .send()?;

        if response.status() == StatusCode::GONE {
            bail!("The issue tracker is disabled on the repository");
        } else if response.status() != StatusCode::CREATED {
            bail!(
                "Failed to create an issue - status code {}",
                response.status().as_u16()
            );
        }

        #[derive(Deserialize)]
        struct IssueCreationResponse {
            // Ignore all unnecessary fields
            url: String,
        }

        let res_body: IssueCreationResponse = response.json()?;
        Ok(res_body.url)
    }
}
