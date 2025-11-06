use derive_more::{Display, From};

use crate::IpcError;

#[derive(Debug, Display, From)]
#[display("{self:?}")]
pub enum Error {
	#[from]
	IpcError(IpcError),
}

// region:    --- Error Boilerplate

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
