use crate::{
	Commit,
	error::{Error, Result},
};
use git2::{Oid, Repository, Revwalk, Sort};
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

	pub fn list_commits_in_branch(&self, branch: &str) -> Result<Vec<Commit>> {
		let branch = self.inner.find_branch(branch, git2::BranchType::Local)?;
		let branch = branch.into_reference();

		let id = branch.target().ok_or_else(|| Error::InvalidBranchTarget)?;
		let mut revwalk = self.inner.revwalk()?;
		revwalk.push(id)?;
		revwalk.set_sorting(Sort::TIME)?;

		let mut commits = Vec::new();
		for commit_id in revwalk {
			let commit_id = commit_id?;
			let commit = self.find_commit(commit_id)?;
			commits.push(commit);
		}
		Ok(commits)
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
