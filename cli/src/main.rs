pub mod cmd;
pub mod poc;
pub mod prelude;

use crate::cmd::{cmd_add, cmd_run, AddArgs, RunArgs};
use crate::prelude::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Rudra PoC utility")]
enum Command {
    #[structopt(about = "Adds new PoC to the directory")]
    Add(AddArgs),
    #[structopt(about = "Runs specified PoC and checks the result")]
    Run(RunArgs),
}

fn update_env() {
    use std::env::{set_var, var};

    // Override rustc to enable the build cache
    set_var("RUSTC_WRAPPER", "sccache");

    // Set LD_LIBRARY_PATH
    const LD_LIBRARY_PATH: &str = "LD_LIBRARY_PATH";
    let dependency_path = PROJECT_PATH.join("dependencies");
    match var(LD_LIBRARY_PATH) {
        Ok(path) => set_var(
            LD_LIBRARY_PATH,
            format!("{}:{}", path, dependency_path.display()),
        ),
        Err(_) => set_var(LD_LIBRARY_PATH, dependency_path),
    }

    // Set RUSTFLAGS
    const RUSTFLAGS: &str = "RUSTFLAGS";
    match var(RUSTFLAGS) {
        Ok(flags) => set_var(RUSTFLAGS, format!("{} -A warnings", flags)),
        Err(_) => set_var(RUSTFLAGS, "-A warnings"),
    }
}

fn main() -> Result<()> {
    update_env();

    let cmd = Command::from_args();
    let result = match cmd {
        Command::Add(args) => cmd_add(args),
        Command::Run(args) => cmd_run(args),
    };

    result
}
