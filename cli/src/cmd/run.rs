use crate::prelude::*;

use crate::poc::PocMap;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct RunArgs {
    #[structopt(parse(try_from_str), help = "PoC ID (4 digits)")]
    id: PocId,
    #[structopt(long, help = "Prepares `poc-debug` directory for PoC development")]
    copy: bool,
}

pub fn cmd_run(args: RunArgs) -> Result<()> {
    let poc_map = PocMap::new()?;

    let metadata = poc_map.read_metadata(args.id)?;

    todo!()
}
