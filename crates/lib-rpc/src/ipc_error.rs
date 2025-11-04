use rpc_router::{RpcError, RpcErrorResponse, RpcId, RpcResponse};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

#[derive(Debug, Clone)]
pub struct IpcError {
	pub id: RpcId,
	pub error: RpcError,
}

impl Serialize for IpcError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let error_response = RpcErrorResponse {
			id: self.id.clone(),
			error: self.error.clone(),
		};

		let rpc_response = RpcResponse::Error(error_response);
		rpc_response.serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for IpcError {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let rpc_response = RpcResponse::deserialize(deserializer)?;

		match rpc_response {
			RpcResponse::Error(err) => Ok(IpcError {
				id: err.id,
				error: err.error,
			}),
			RpcResponse::Success(success) => Err(DeError::custom(format!(
				"Expected an error response, but got a success response: id={}",
				success.id
			))),
		}
	}
}
