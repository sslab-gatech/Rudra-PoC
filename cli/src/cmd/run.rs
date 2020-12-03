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

    // Workspace preparation
    let temp_dir = TempDir::new("rudra-poc").context("Failed to create a temp directory")?;

    let workspace_path = if args.debug {
        PROJECT_PATH.join("poc-debug")
    } else {
        temp_dir.path().to_path_buf()
    };

    poc_map.prepare_poc_workspace(args.id, &workspace_path)?;

    // cargo run
    let metadata = poc_map.read_metadata(args.id)?;

    let mut cmd = util::cargo_command("build", &metadata);
    if !cmd.current_dir(&workspace_path).spawn()?.wait()?.success() {
        anyhow::bail!("`cargo build` failed");
    }

    let mut cmd = util::cargo_command("run", &metadata);
    let exit_status = cmd.current_dir(&workspace_path).spawn()?.wait()?;
    println!("\n{}", util::exit_status_string(&exit_status));

    Ok(())
}
