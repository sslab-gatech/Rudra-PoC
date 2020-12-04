use std::fs;

use crate::git::GitClient;
use crate::poc::PocMap;
use crate::prelude::*;

use askama::Template;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct SyncArgs {}

#[derive(Template)]
#[template(path = "README.md")]
pub struct ReadmeTemplate {
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

pub struct ReadmeTemplateLine {
    poc_id: PocId,
    krate: String,
    analyzers: Vec<String>,
    issue_url: Option<String>,
    rustsec_id: Option<String>,
}

mod filters {
    pub fn unordered_list(vec: &Vec<String>) -> askama::Result<String> {
        let mut s = String::new();
        s.push_str("<ul>");
        for analyzer_name in vec.iter() {
            s.push_str("<li>");
            s.push_str(analyzer_name);
            s.push_str("</li>");
        }
        s.push_str("</ul>");
        Ok(s)
    }

    pub fn na(msg: &Option<String>) -> askama::Result<&str> {
        match msg {
            Some(msg) => Ok(msg),
            None => Ok("N/A"),
        }
    }
}

pub fn cmd_sync(_args: SyncArgs) -> Result<()> {
    let poc_map = PocMap::new()?;

    let mut readme_template = ReadmeTemplate::new();
    for poc_id in poc_map.iter_ids() {
        let metadata = poc_map.read_metadata(poc_id)?;

        let krate = metadata.target.krate.clone();
        let analyzers: Vec<_> = metadata
            .test
            .analyzers
            .iter()
            .map(|analyzer| analyzer.to_string())
            .collect();
        let issue_url = metadata.report.issue_url.clone();
        let rustsec_id = metadata.report.rustsec_id.clone();

        let line = ReadmeTemplateLine {
            poc_id,
            krate,
            analyzers,
            issue_url,
            rustsec_id,
        };

        readme_template.push(line);
    }

    let readme_content = readme_template.render()?;
    fs::write(PROJECT_PATH.join("README.md"), readme_content)?;

    // TODO: detect and update RUSTSEC ID from git repository
    let git_client = GitClient::new_with_config_file()?;
    let _repository = git_client.prepare_rustsec_local()?;

    todo!()
}
