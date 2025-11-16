use crate::{Ctx, RepoManager, error::Result, fire_event};
use lib_git::{Branch, Repo};

pub struct RepoBmc;

impl RepoBmc {
	pub fn open_repo(_rm: &RepoManager, ctx: &Ctx, path: String) -> Result<Repo> {
		let repo = Repo::open(path)?;
		fire_event(&ctx, "Model", "repo", "open", repo.root());
		Ok(repo)
	}

	pub fn list_branches(rm: &RepoManager) -> Result<Vec<Branch>> {
		let repo_guard = rm.get_repo()?;
		let repo = repo_guard.get()?;
		let branches = repo.list_branches()?;
		Ok(branches)
	}
}
