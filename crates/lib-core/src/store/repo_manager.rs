use std::sync::{Arc, Mutex, MutexGuard};

use lib_git::Repo;
use rpc_router::RpcResource;

use crate::error::{Error, Result};

#[derive(Default, Clone, RpcResource)]
pub struct RepoManager {
	repo: Arc<Mutex<Option<Repo>>>,
}

pub struct RepoGuard<'a> {
	guard: MutexGuard<'a, Option<Repo>>,
}

impl<'a> RepoGuard<'a> {
	pub fn get(&self) -> Result<&Repo> {
		self.guard.as_ref().ok_or(Error::NoRepoOpened)
	}
}

impl RepoManager {
	// pub fn get_repo(&self) -> Result<Repo> {
	// 	let guard = self.repo.lock()?;
	// 	let repo = guard.as_ref().ok_or(Error::NoRepoOpened)?;
	// 	Ok(repo.clone())
	// }

	pub fn get_repo(&self) -> Result<RepoGuard<'_>> {
		let guard = self.repo.lock()?;
		Ok(RepoGuard { guard })
	}

	pub fn set_repo(&self, repo: Repo) -> Result<()> {
		let mut guard = self.repo.lock()?;
		*guard = Some(repo);
		Ok(())
	}

	pub fn clear(&self) -> Result<()> {
		let mut repo_lock = self.repo.lock()?;
		*repo_lock = None;
		Ok(())
	}
}

impl From<Repo> for RepoManager {
	fn from(value: Repo) -> Self {
		Self {
			repo: Arc::new(Mutex::new(Some(value))),
		}
	}
}
