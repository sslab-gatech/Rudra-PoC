use std::{collections::HashSet, fs};

use crate::common::get_poc_id_set;
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

fn find_next_poc_id(id_set: &HashSet<PocId>) -> PocId {
    for id in PocId::iter_all() {
        if !id_set.contains(&id) {
            return id;
        }
    }

    panic!("No more PoC can be added!");
}

pub fn add_cmd(args: AddArgs) -> Result<()> {
    // Parse existing PoC IDs
    let id_set = get_poc_id_set()?;
    let new_poc_id = find_next_poc_id(&id_set);
    let new_poc_name = format!("{}-{}", new_poc_id, args.krate);
    let new_poc_path = PROJECT_PATH.join(format!("poc/{}.rs", &new_poc_name));

    // Create a new PoC file
    fs::write(&new_poc_path, args.render().unwrap())
        .with_context(|| format!("Failed to write to {}", new_poc_path.display()))?;

    // TODO: Setup poc-debug directory

    println!("Successfully added {}", new_poc_name);

    Ok(())
}
