use crate::{
	Branch, Commit,
	error::{Error, Result, parse_fail},
};
use gix::{
	ObjectId, ThreadSafeRepository,
	bstr::BString,
	create::Options,
	diff::index::Change,
	progress,
	revision::walk::Sorting,
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
	pub summary: String, // e.g. +, -, ~, ->
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

	pub fn name(&self) -> String {
		self.inner
			.work_dir()
			.and_then(|p| p.file_name())
			.and_then(|n| n.to_str())
			.unwrap_or("<unknown>")
			.to_string()
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

	pub fn list_commits_in_branch(&self, branch: &str) -> Result<Vec<Commit>> {
		let repo = self.inner.to_thread_local();
		let reference = repo
			.find_reference(format!("refs/heads/{branch}").as_str())
			.map_err(parse_fail)?;

		// TODO: split into multiple lines without ownership issues
		let revwalk = repo.rev_walk([reference.target().id()]).sorting(Sorting::ByCommitTime(
			gix::traverse::commit::simple::CommitTimeOrder::NewestFirst,
		));

		let mut commits = Vec::new();
		for info in revwalk.all()? {
			let info = info?;
			let commit = repo.find_commit(info.id)?;
			let message = commit.message().map_err(parse_fail)?;
			let author = commit.author().map_err(parse_fail)?;
			let committer = commit.committer().map_err(parse_fail)?;

			let time = committer.time().map_err(parse_fail)?.seconds;

			commits.push(Commit {
				id: commit.id,
				tree: commit.tree_id().map_err(parse_fail)?.into(),
				message: message.body.unwrap_or("<unknown>".into()).to_string(),
				author: author.name.to_string(),
				time,
			});
		}
		Ok(commits)
	}

	pub fn find_commit(&self, id: ObjectId) -> Result<Commit> {
		let repo = self.inner.to_thread_local();
		let commit = repo.find_object(id)?.try_into_commit()?;
		let tree_id = commit.tree_id().map_err(parse_fail)?;
		let message = commit.message().map_err(parse_fail)?;
		let author = commit.author().map_err(parse_fail)?;
		let committer = commit.committer().map_err(parse_fail)?;
		let time = committer.time().map_err(parse_fail)?.seconds;

		Ok(Commit {
			id: commit.id,
			tree: ObjectId::from(tree_id),
			message: message.body.unwrap_or("<unknown>".into()).to_string(),
			author: author.name.to_string(),
			time,
		})
	}

	pub fn head_commit(&self) -> Result<Commit> {
		let repo = self.inner.to_thread_local();

		let head = repo.head_commit()?;
		let tree_id = head.tree_id().map_err(parse_fail)?;
		let message = head.message().map_err(parse_fail)?;
		let author = head.author().map_err(parse_fail)?;
		let committer = head.committer().map_err(parse_fail)?;
		let time = committer.time().map_err(parse_fail)?.seconds;
		Ok(Commit {
			id: head.id,
			tree: ObjectId::from(tree_id),
			message: message.body.unwrap_or("<unknown>".into()).to_string(),
			author: author.name.to_string(),
			time,
		})
	}
}
