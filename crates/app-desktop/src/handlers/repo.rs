use std::sync::Arc;

use crate::Result;
use lib_core::{Ctx, RepoBmc, RepoManager};
use lib_rpc::ParamsForOpen;

// TODO
pub fn open_repo(rm: RepoManager, ctx: Arc<Ctx>, params: ParamsForOpen) -> Result<()> {
	let repo = RepoBmc::open_repo(ctx, params.path)?;
	rm.set_repo(repo)?;
	Ok(())
}
