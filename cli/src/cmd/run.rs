use crate::poc::PocMap;
use crate::prelude::*;

use structopt::StructOpt;
use tempdir::TempDir;

#[derive(Debug, StructOpt)]
pub struct RunArgs {
    #[structopt(parse(try_from_str), help = "PoC ID (4 digits)")]
    poc_id: PocId,
    #[structopt(
        short,
        long,
        help = "Prepares `poc-debug` directory for PoC development"
    )]
    debug: bool,
}

pub fn cmd_run(args: RunArgs) -> Result<()> {
    let poc_map = PocMap::new()?;

    // Workspace preparation
    let temp_dir = TempDir::new("rudra-poc").context("Failed to create a temp directory")?;

    let workspace_path = if args.debug {
        PROJECT_PATH.join("poc-debug")
    } else {
        temp_dir.path().to_path_buf()
    };

    poc_map.prepare_poc_workspace(args.poc_id, &workspace_path)?;

    // cargo run
    let metadata = poc_map.read_metadata(args.poc_id)?;

    let cmd = util::cargo_command("build", &metadata.test, &workspace_path);
    cmd.run()?;

    let cmd = util::cargo_command("run", &metadata.test, &workspace_path).unchecked();
    println!("\n{}", util::exit_status_string(&cmd.run()?.status));

    Ok(())
}
