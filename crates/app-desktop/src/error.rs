use std::string::FromUtf8Error;

use derive_more::{Display, From};
use rpc_router::RpcHandlerError;
use serde::Serialize;
use serde_json::Value;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, RpcHandlerError, Display, From)]
#[display("{self:?}")]
pub enum Error {
	#[from(String, &String, &str)]
	Custom(String),
	CtxFail,
	#[from]
	JsonSerde(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
	#[from]
	Tauri(#[serde_as(as = "DisplayFromStr")] tauri::Error),
	#[from]
	LibCore(lib_core::Error),

	StrongholdCredentialsNotFound,
	StrongholdCredentialsNotUtf8,
	VaultNotInitialized,
	#[from]
	IotaStronghold(#[serde_as(as = "DisplayFromStr")] iota_stronghold::ClientError),
	#[from]
	IotaMemory(#[serde_as(as = "DisplayFromStr")] iota_stronghold::MemoryError),

	#[from]
	IO(#[serde_as(as = "DisplayFromStr")] std::io::Error),
	StrongholdStoreFail(String),
	MutexPoison,
	VaultPathNotFound,
	#[from]
	RpcRequestParsing(rpc_router::RpcRequestParsingError),
	#[from]
	RpcLibRpc(lib_rpc::Error),
	RpcHandlerErrorUnhandled(&'static str),
	RpcRouter {
		id: Box<Value>,
		method: String,
		error: rpc_router::Error,
	},
}

impl<T> From<std::sync::PoisonError<T>> for Error {
	fn from(_val: std::sync::PoisonError<T>) -> Self {
		Self::MutexPoison
	}
}

impl From<rpc_router::CallError> for Error {
	fn from(call_error: rpc_router::CallError) -> Self {
		let rpc_router::CallError { id, method, error } = call_error;
		match error {
			rpc_router::Error::Handler(mut rpc_handler_error) => {
				if let Some(lib_rpc_error) = rpc_handler_error.remove::<lib_rpc::Error>() {
					Error::RpcLibRpc(lib_rpc_error)
				} else {
					let type_name = rpc_handler_error.type_name();
					Error::RpcHandlerErrorUnhandled(type_name)
				}
			}
			error => Error::RpcRouter {
				id: Box::new(id.to_value()),
				method,
				error,
			},
		}
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
