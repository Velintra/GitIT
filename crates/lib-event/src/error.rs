use derive_more::From;
use flume::{RecvError, SendError};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
	#[from(String, &String, &str)]
	Custom(String),

	FlumeErrorWrapper {
		message: String,
	},
	#[from]
	SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
	EventSend(String),
	EventRecv(#[serde_as(as = "DisplayFromStr")] RecvError),
}

impl<T> From<SendError<T>> for Error {
	fn from(value: SendError<T>) -> Self {
		Self::EventSend(value.to_string())
	}
}

impl From<RecvError> for Error {
	fn from(err: RecvError) -> Self {
		Self::EventRecv(err)
	}
}

// region: --- Custom

impl Error {
	pub fn custom_from_err(err: impl std::error::Error) -> Self {
		Self::Custom(err.to_string())
	}

	pub fn custom(val: impl Into<String>) -> Self {
		Self::Custom(val.into())
	}
}

// endregion: --- Custom

impl core::fmt::Display for Error {
	fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
