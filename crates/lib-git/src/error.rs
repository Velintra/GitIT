use derive_more::{Display, From};
use gix::reference;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Display, From)]
#[display("{self:?}")]
pub enum Error {
	#[from(String, &String, &str)]
	Custom(String),
	// -- Externals
	InvalidBranchTarget,
	GixObjectParseFail(String),
	BranchNotExist,
	#[from]
	GixDiscover(#[serde_as(as = "DisplayFromStr")] gix::discover::Error),
	#[from]
	GixInit(#[serde_as(as = "DisplayFromStr")] gix::init::Error),
	#[from]
	GixFind(#[serde_as(as = "DisplayFromStr")] gix::object::find::existing::Error),
	#[from]
	GixTryInto(#[serde_as(as = "DisplayFromStr")] gix::object::try_into::Error),
	#[from]
	GixCommit(#[serde_as(as = "DisplayFromStr")] gix::object::commit::Error),
	#[from]
	GixReference(#[serde_as(as = "DisplayFromStr")] gix::reference::head_commit::Error),
	#[from]
	GixStatus(#[serde_as(as = "DisplayFromStr")] gix::status::Error),
	#[from]
	GixRef(#[serde_as(as = "DisplayFromStr")] gix::reference::iter::init::Error),
	#[from]
	GixRefOpen(#[serde_as(as = "DisplayFromStr")] gix::refs::packed::buffer::open::Error),
	#[from]
	GixStatusIntoIter(#[serde_as(as = "DisplayFromStr")] gix::status::into_iter::Error),
	#[from]
	GixStatusIter(#[serde_as(as = "DisplayFromStr")] gix::status::iter::Error),
	#[from]
	GixRevisionWalk(#[serde_as(as = "DisplayFromStr")] gix::revision::walk::Error),
	#[from]
	GixRevisionWalkIter(#[serde_as(as = "DisplayFromStr")] gix::revision::walk::iter::Error),
	#[from]
	GixFindExisting(#[serde_as(as = "DisplayFromStr")] gix::object::find::existing::with_conversion::Error),
	#[from]
	GixRefNameError(#[serde_as(as = "DisplayFromStr")] gix::validate::reference::name::Error), // #[from]
	#[from]
	GixRefEditError(#[serde_as(as = "DisplayFromStr")] gix::reference::edit::Error), // Git2(#[serde_as(as = "DisplayFromStr")] git2::Error),
	#[from]
	GixFindReferror(#[serde_as(as = "DisplayFromStr")] gix::reference::find::existing::Error),
}

pub(crate) fn parse_fail<E: std::fmt::Display>(err: E) -> Error {
	Error::GixObjectParseFail(err.to_string())
}

// region:    --- Custom

impl Error {
	pub fn custom_from_err(err: impl std::error::Error) -> Self {
		Self::Custom(err.to_string())
	}

	pub fn custom(val: impl Into<String>) -> Self {
		Self::Custom(val.into())
	}
}

// endregion: --- Custom

// region:    --- Error Boilerplate

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
