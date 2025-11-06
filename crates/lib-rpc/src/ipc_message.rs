use derive_more::From;
use rpc_router::RpcId;
use serde::{Serialize, Serializer};
use serde_json::Value;

use crate::{IpcError, IpcResponse};

#[derive(Debug, Clone, From)]
pub enum IpcMessage {
	#[from]
	Response(IpcResponse<Value>),
	#[from]
	Error(IpcError),
}

impl IpcMessage {
	pub fn rpc_id(&self) -> Option<&RpcId> {
		match self {
			IpcMessage::Response(resp) => Some(&resp.id),
			IpcMessage::Error(err) => Some(&err.id),
		}
	}
}

impl Serialize for IpcMessage {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			IpcMessage::Response(res) => res.serialize(serializer),
			IpcMessage::Error(err) => err.serialize(serializer),
		}
	}
}
