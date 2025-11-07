use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum Error {
	#[from(String, &String, &str)]
	Custom(String),
	// -- Externals
	InvalidBranchTarget,
	#[from]
	GixDiscover(gix::discover::Error),
	#[from]
	GixInit(gix::init::Error),
	#[from]
	GixFind(gix::object::find::existing::Error),
	#[from]
	GixTryInto(gix::object::try_into::Error),
	#[from]
	GixCommit(gix::object::commit::Error),
	#[from]
	GixReference(gix::reference::head_commit::Error),
	#[from]
	GixStatus(gix::status::Error),

	#[from]
	GixRef(gix::reference::iter::init::Error),
	#[from]
	GixRefOpen(gix::refs::packed::buffer::open::Error),
	#[from]
	GixStatusIntoIter(gix::status::into_iter::Error),
	#[from]
	GixStatusIter(gix::status::iter::Error),
	#[from]
	Git2(git2::Error),
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
