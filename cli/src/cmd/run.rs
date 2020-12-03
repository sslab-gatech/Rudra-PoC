use crate::poc::PocMap;
use crate::prelude::*;

use structopt::StructOpt;
use tempdir::TempDir;

#[derive(Debug, StructOpt)]
pub struct RunArgs {
    #[structopt(parse(try_from_str), help = "PoC ID (4 digits)")]
    id: PocId,
    #[structopt(
        short,
        long,
        help = "Prepares `poc-debug` directory for PoC development"
    )]
    debug: bool,
}

pub fn cmd_run(args: RunArgs) -> Result<()> {
    let poc_map = PocMap::new()?;

    let temp_dir = TempDir::new("rudra-poc").context("Failed to create a temp directory")?;

    let workspace_path = if args.debug {
        PROJECT_PATH.join("poc-debug")
    } else {
        temp_dir.path().to_path_buf()
    };

    poc_map.prepare_poc_workspace(args.id, &workspace_path)?;

    todo!()
}
