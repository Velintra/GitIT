use crate::{Ctx, RepoManager, error::Result, fire_event};
use lib_git::{Branch, Repo};
use rpc_router::RpcParams;
use serde::Deserialize;

pub struct RepoBmc;

// TODO: Generalize params in some way

#[derive(Deserialize, RpcParams)]
pub struct BranchForCreate {
	pub name: String,
}

impl RepoBmc {
	pub fn open_repo(_rm: &RepoManager, ctx: &Ctx, path: String) -> Result<Repo> {
		let repo = Repo::open(path)?;
		fire_event(&ctx, "Model", "repo", "open", repo.root());
		Ok(repo)
	}

	pub fn create_branch(rm: &RepoManager, ctx: &Ctx, name: String) -> Result<String> {
		let repo_guard = rm.get_repo()?;
		let repo = repo_guard.get()?;
		let id = repo.create_branch(&name)?;
		fire_event(&ctx, "Model", "branch", "create", id);

		Ok(id.to_string())
	}

	pub fn list_branches(rm: &RepoManager) -> Result<Vec<Branch>> {
		let repo_guard = rm.get_repo()?;
		let repo = repo_guard.get()?;
		let branches = repo.list_branches()?;
		Ok(branches)
	}
}
