use std::sync::Arc;

use crate::{Ctx, error::Result, fire_model_event};
use lib_git::Repo;

pub struct RepoBmc;

impl RepoBmc {
	pub fn open_repo(ctx: Arc<Ctx>, path: String) -> Result<Repo> {
		let repo = Repo::open(path)?;
		fire_model_event(&ctx, "repo", "open", repo.root());
		Ok(repo)
	}
}
