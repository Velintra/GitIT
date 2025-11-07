use crate::Result;
use lib_core::{Ctx, RepoBmc, RepoManager};
use lib_rpc::{DataIpcResult, ParamsForOpen};
use rpc_router::{router_builder, RouterBuilder};

pub fn router_builder() -> RouterBuilder {
	router_builder!(open_repo)
}

pub async fn open_repo(ctx: Ctx, rm: RepoManager, params: ParamsForOpen) -> Result<DataIpcResult<String>> {
	let ParamsForOpen { path } = params;

	let repo = RepoBmc::open_repo(ctx.into(), path)?;
	let root = repo.root();
	rm.set_repo(repo)?;
	Ok(root.into())
}
