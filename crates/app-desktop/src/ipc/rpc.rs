use crate::{Error, Result};
use lib_core::Ctx;
use lib_rpc::{IpcError, IpcMessage, IpcResponse};
use rpc_router::resources_builder;
use serde_json::Value;
use tauri::{AppHandle, State, Wry};

#[tauri::command]
pub async fn rpc_handler(
	app_handle: AppHandle<Wry>,
	rpc_router: State<'_, rpc_router::Router>,
	rpc_req: Value,
) -> Result<IpcMessage> {
	let rpc_req = match rpc_router::RpcRequest::try_from(rpc_req) {
		Ok(rpc_req) => rpc_req,
		Err(rpc_req_error) => {
			return Err(Error::RpcRequestParsing(rpc_req_error));
		}
	};

	let ctx = Ctx::from_app(app_handle)?;

	let additional_resources = resources_builder![ctx].build();

	let rpc_call_res = rpc_router.call_with_resources(rpc_req, additional_resources).await;

	match rpc_call_res {
		Ok(rpc_call_response) => {
			let response = IpcResponse {
				id: rpc_call_response.id,
				result: rpc_call_response.value,
			};
			Ok(IpcMessage::Response(response))
		}
		Err(err) => {
			let ipc_error = IpcError {
				id: err.id.clone(),
				error: err.into(),
			};
			Ok(IpcMessage::Error(ipc_error))
		}
	}
}
