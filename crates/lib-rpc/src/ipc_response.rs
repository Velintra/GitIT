use rpc_router::{RpcId, RpcResponse, RpcSuccessResponse};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct IpcResponse<R = Value> {
	pub id: RpcId,
	pub result: R,
}

impl<R> Serialize for IpcResponse<R>
where
	R: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let result_val = serde_json::to_value(&self.result).map_err(serde::ser::Error::custom)?;

		let success_response = RpcSuccessResponse {
			id: self.id.clone(),
			result: result_val,
		};

		let rpc_response = RpcResponse::Success(success_response);
		rpc_response.serialize(serializer)
	}
}

impl<'de, R> Deserialize<'de> for IpcResponse<R>
where
	R: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let rpc_response = RpcResponse::deserialize(deserializer)?;

		match rpc_response {
			RpcResponse::Success(success) => {
				let result = R::deserialize(success.result).map_err(DeError::custom)?;
				Ok(IpcResponse { id: success.id, result })
			}
			RpcResponse::Error(err) => Err(DeError::custom(format!(
				"Expected a success response, but got an error response: id={}, code={}, message='{}'",
				err.id, err.error.code, err.error.message
			))),
		}
	}
}
