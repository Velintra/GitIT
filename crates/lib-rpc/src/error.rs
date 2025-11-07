use derive_more::{Display, From};
use rpc_router::RpcHandlerError;
use serde::Serialize;

use crate::IpcError;

#[derive(Debug, Display, Serialize, From)]
#[display("{self:?}")]
pub enum Error {
	#[from]
	IpcError(IpcError),
}

// region:    --- Error Boilerplate

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
