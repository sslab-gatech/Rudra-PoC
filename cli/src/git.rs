use std::{fs, path::PathBuf};

use crate::prelude::*;

use reqwest::{blocking::Client, header};
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

pub struct GitClient {
    auth_config: AuthConfig,
    github_client: Client,
}

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

    pub fn github_remote_callbacks(&self) -> git2::RemoteCallbacks<'_> {
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
}
