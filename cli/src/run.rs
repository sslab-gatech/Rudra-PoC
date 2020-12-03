use crate::prelude::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct RunArgs {
    #[structopt(parse(try_from_str), help = "PoC ID (4 digits)")]
    id: PocId,
    #[structopt(long, help = "Prepares `poc-debug` directory for PoC development")]
    copy: bool,
}

pub fn run_cmd(args: RunArgs) -> Result<()> {
    todo!()
}
