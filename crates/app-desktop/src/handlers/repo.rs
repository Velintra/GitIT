use crate::Result;
use lib_core::{BranchForCreate, Ctx, RepoBmc, RepoManager};
use lib_git::Branch;
use lib_rpc::{DataIpcResult, ParamsForOpen, ParamsStringed};
use rpc_router::{router_builder, RouterBuilder};

pub fn router_builder() -> RouterBuilder {
	router_builder!(open_repo, list_branches, create_branch, delete_branch)
}

pub async fn open_repo(rm: RepoManager, ctx: Ctx, params: ParamsStringed) -> Result<DataIpcResult<String>> {
	let ParamsStringed { data } = params;

	let repo = RepoBmc::open_repo(&rm, &ctx, data)?;
	let root = repo.name();
	rm.set_repo(repo)?;
	Ok(root.into())
}

pub async fn create_branch(rm: RepoManager, ctx: Ctx, params: ParamsStringed) -> Result<DataIpcResult<String>> {
	let ParamsStringed { data } = params;

	let id = RepoBmc::create_branch(&rm, &ctx, data)?;

	Ok(id.into())
}

pub async fn delete_branch(rm: RepoManager, ctx: Ctx, params: ParamsStringed) -> Result<DataIpcResult<String>> {
	let ParamsStringed { data } = params;

	let id = RepoBmc::delete_branch(&rm, &ctx, data)?;

	Ok(id.into())
}

pub async fn list_branches(rm: RepoManager, _ctx: Ctx) -> Result<DataIpcResult<Vec<Branch>>> {
	let branches = RepoBmc::list_branches(&rm)?;
	Ok(branches.into())
}
