use derive_more::{Display, From};
use serde::Serialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Display, From)]
#[display("{self:?}")]
pub enum Error {
	#[from(String, &String, &str)]
	Custom(String),
	MutexPoison,
	VaultNotInitialized,
	NoRepoOpened,
	#[from]
	IotaStronghold(#[serde_as(as = "DisplayFromStr")] iota_stronghold::ClientError),
	#[from]
	LibGit(lib_git::Error),
	// -- Externals
}

impl<T> From<std::sync::PoisonError<T>> for Error {
	fn from(_val: std::sync::PoisonError<T>) -> Self {
		Self::MutexPoison
	}
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
