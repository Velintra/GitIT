use crate::{
	Commit,
	error::{Error, Result},
};
use git2::{Oid, Repository, Sort};
use std::{path::Path, sync::Arc};

pub struct Repo {
	inner: Arc<Repository>,
}

impl Clone for Repo {
	fn clone(&self) -> Self {
		Self {
			inner: Arc::clone(&self.inner),
		}
	}
}

impl Repo {
	pub fn open(path: impl AsRef<Path>) -> Result<Repo> {
		Ok(Self {
			inner: Arc::new(Repository::discover(path)?),
		})
	}

	pub fn init(path: impl AsRef<Path>) -> Result<Self> {
		Ok(Self {
			inner: Arc::new(Repository::init(path)?),
		})
	}

	pub fn list_branches(&self) -> Result<Vec<String>> {
		let branches = self.inner.branches(None)?;
		let mut names = Vec::new();
		for branch in branches {
			let (branch, _) = branch?;
			if let Some(name) = branch.name()? {
				names.push(name.to_string());
			}
		}
		Ok(names)
	}

	pub fn root(&self) -> String {
		self.inner.path().display().to_string()
	}

	pub fn status(&self) -> Result<Vec<String>> {
		let statuses = self.inner.statuses(None)?;
		let mut files = Vec::new();
		for entry in statuses.iter() {
			if let Some(path) = entry.path() {
				files.push(path.to_string());
			}
		}
		Ok(files)
	}

	pub fn list_commits_in_branch(&self, branch: &str) -> Result<Vec<Commit>> {
		let branch = self.inner.find_branch(branch, git2::BranchType::Local)?;
		let branch = branch.into_reference();

		let oid = branch.target().ok_or(Error::InvalidBranchTarget)?;
		let mut revwalk = self.inner.revwalk()?;
		revwalk.push(oid)?;
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
