use crate::{Commit, error::Result};
use git2::{Oid, Repository};
use std::path::Path;

pub struct Repo {
	inner: Repository,
}

impl Repo {
	pub fn open(path: impl AsRef<Path>) -> Result<Repo> {
		Ok(Self {
			inner: Repository::discover(path)?,
		})
	}

	pub fn find_commit(&self, commit_id: Oid) -> Result<Commit> {
		let commit = self.inner.find_commit(commit_id)?;
		let tree = commit.tree()?;
		Ok(Commit {
			id: commit.id(),
			tree: tree.id(),
			message: commit.message().unwrap_or("<unknown>").to_string(),
			author: commit.author().name().unwrap_or("<unknown>").to_string(),
			time: commit.time().seconds(),
		})
	}

	pub fn head_commit(&self) -> Result<Commit> {
		let head = self.inner.head()?.peel_to_commit()?;
		self.find_commit(head.id())
	}
}
