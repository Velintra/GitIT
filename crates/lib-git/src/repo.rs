use crate::{
	Branch,
	error::{Error, Result},
};
use gix::{
	ObjectId, ThreadSafeRepository,
	bstr::BString,
	create::Options,
	diff::index::Change,
	progress,
	status::{UntrackedFiles, index_worktree::iter::Summary},
};

use gix::status::Item;
use serde::Serialize;
use std::{
	path::Path,
	sync::{Arc, atomic::AtomicBool},
};

pub struct Repo {
	inner: ThreadSafeRepository,
}

#[derive(Debug, Serialize)]
pub struct FileStatus {
	pub path: String,
	pub summary: String, // e.g. Modified - "M " "A " "??"
}

impl Repo {
	pub fn open(path: impl AsRef<Path>) -> Result<Repo> {
		Ok(Self {
			inner: ThreadSafeRepository::discover(path)?,
		})
	}

	pub fn init(path: impl AsRef<Path>) -> Result<Self> {
		Ok(Self {
			inner: ThreadSafeRepository::init(
				path,
				gix::create::Kind::WithWorktree,
				Options {
					destination_must_be_empty: false,
					..Default::default()
				},
			)?,
		})
	}

	pub fn list_branches(&self) -> Result<Vec<Branch>> {
		let repo = self.inner.to_thread_local();
		let mut branches = Vec::new();

		// Local
		let refs = repo.references()?;
		let heads = refs.prefixed("refs/heads/")?;
		let peeled = heads.peeled()?;

		for reference in peeled {
			let reference = reference.map_err(|err| Error::Custom(err.to_string()))?;
			let name = reference.name().shorten();
			let target = ObjectId::from(reference.target().id());

			branches.push(Branch {
				name: name.to_string(),
				kind: "Local".into(),
				target,
			});
		}

		// TODO: Remote branches

		Ok(branches)
	}

	pub fn root(&self) -> String {
		self.inner.path().display().to_string()
	}

	pub fn status(&self) -> Result<Vec<FileStatus>> {
		let repo = self.inner.to_thread_local();

		let mut iter = repo
			.status(progress::Discard)?
			.untracked_files(UntrackedFiles::None)
			.should_interrupt_owned(Arc::new(AtomicBool::new(false)))
			.into_iter(std::iter::empty::<BString>())?;

		let mut files = Vec::new();

		while let Some(item) = iter.next() {
			let item = item?;

			match item {
				Item::IndexWorktree(change) => {
					if let Some(summary) = change.summary() {
						let path = change.rela_path().to_string();
						let code = match summary {
							Summary::Added => "+",
							Summary::Removed => "-",
							Summary::Modified => "~",
							Summary::Renamed => "→",
							Summary::Copied => "≈",
							Summary::TypeChange => "⇄",
							Summary::IntentToAdd => "✚",
							Summary::Conflict => "⚡",
						};
						files.push(FileStatus {
							path,
							summary: code.to_string(),
						});
					}
				}

				Item::TreeIndex(change) => {
					let path = change.location().to_string();

					let code = match change {
						Change::Addition { .. } => "+",
						Change::Modification { .. } => "~",
						Change::Deletion { .. } => "-",
						Change::Rewrite { .. } => "→",
					};

					files.push(FileStatus {
						path,
						summary: code.to_string(),
					});
				}
			}
		}

		Ok(files)
	}

	// pub fn list_commits_in_branch(&self, branch: &str) -> Result<Vec<Commit>> {
	// 	let branch = self.inner.find_branch(branch, git2::BranchType::Local)?;
	// 	let branch = branch.into_reference();

	// 	let oid = branch.target().ok_or(Error::InvalidBranchTarget)?;
	// 	let mut revwalk = self.inner.revwalk()?;
	// 	revwalk.push(oid)?;
	// 	revwalk.set_sorting(Sort::TIME)?;

	// 	let mut commits = Vec::new();
	// 	for commit_id in revwalk {
	// 		let commit_id = commit_id?;
	// 		let commit = self.find_commit(commit_id)?;
	// 		commits.push(commit);
	// 	}
	// 	Ok(commits)
	// }

	// pub fn find_commit(&self, id: gix::ObjectId) -> Result<Commit> {
	// 	let repo = self.inner.to_thread_local();
	// 	let commit = repo.find_object(id)?.try_into_commit()?;
	// 	let tree_id = commit.tree_id()?;
	// 	Ok(Commit {
	// 		id: commit.id,
	// 		tree: commit.tree()?.id,
	// 		message: commit.message.unwrap_or("<unknown>".into()),
	// 		author: commit
	// 			.author()
	// 			.map(|a| a.name.unwrap_or("<unknown>".into()))
	// 			.unwrap_or("<unknown>".into()),
	// 		time: commit.committer().map(|c| c.time.seconds_since_unix_epoch).unwrap_or(0),
	// 	})
	// }

	// pub fn head_commit(&self) -> Result<Commit> {
	// 	let repo = self.inner.to_thread_local();

	// 	let head = repo.head_commit()?;
	// 	Ok(Commit {
	// 		id: head.id,
	// 		tree: head.tree_id,
	// 		message: head.message.unwrap_or("<unknown>".into()),
	// 		author: head
	// 			.author()?
	// 			.map(|a| a.name.unwrap_or("<unknown>".into()))
	// 			.unwrap_or("<unknown>".into()),
	// 		time: head.committer.map(|c| c.time.seconds_since_unix_epoch).unwrap_or(0),
	// 	})
	// }
}
