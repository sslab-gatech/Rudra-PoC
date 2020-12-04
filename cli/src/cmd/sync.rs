use crate::git::GitClient;
use crate::poc::PocMap;
use crate::prelude::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct SyncArgs {}

pub fn cmd_sync(args: SyncArgs) -> Result<()> {
    let git_client = GitClient::new_with_config_file()?;
    let _repository = git_client.prepare_rustsec_local()?;

    todo!()
}
