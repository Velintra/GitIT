use rpc_router::RpcParams;
use serde::Deserialize;

#[derive(Deserialize, RpcParams)]
pub struct ParamsForCreate<D> {
	pub data: D,
}

#[derive(Deserialize, RpcParams)]
pub struct ParamsForOpen {
	pub path: String,
}

#[derive(Deserialize, RpcParams)]
pub struct ParamsIded {
	pub id: i64,
}
