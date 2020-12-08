use std::fs;

use crate::poc::PocMap;
use crate::prelude::*;

use askama::Template;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Template)]
#[template(path = "new_poc.rs", escape = "none")]
pub struct AddArgs {
    #[structopt(name = "crate", help = "target crate name")]
    krate: String,
    #[structopt(
        parse(try_from_str = semver::Version::parse),
        help = "target crate version"
    )]
    version: semver::Version,
}

pub fn cmd_add(args: AddArgs) -> Result<()> {
    let poc_map = PocMap::new()?;

    let new_poc_id = poc_map.next_empty_id();
    let new_poc_name = format!("{}-{}", new_poc_id, args.krate);
    let new_poc_path = PROJECT_PATH.join(format!("poc/{}.rs", &new_poc_name));

    // Create a new PoC file
    fs::write(&new_poc_path, args.render().unwrap())
        .with_context(|| format!("Failed to write to {}", new_poc_path.display()))?;

    // Setup poc-debug directory
    let poc_map = PocMap::new()?;
    poc_map.prepare_poc_workspace(new_poc_id, PROJECT_PATH.join("poc-debug"))?;

    println!("Successfully added {}", new_poc_name);

    crate::cmd::update::update_readme()?;

    Ok(())
}
