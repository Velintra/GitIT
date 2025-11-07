use crate::Result;
use lib_core::{Ctx, RepoBmc, RepoManager};
use lib_rpc::{DataIpcResult, ParamsForOpen};

use std::sync::Arc;

pub fn open_repo(rm: RepoManager, ctx: Arc<Ctx>, params: ParamsForOpen) -> Result<DataIpcResult<String>> {
	let repo = RepoBmc::open_repo(ctx, params.path)?;
	let root = repo.root();
	rm.set_repo(repo)?;
	Ok(root.into())
}
