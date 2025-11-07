use rpc_router::{IntoParams, RpcParams};
use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
	pub data: D,
}

impl<D> IntoParams for ParamsForCreate<D> where D: DeserializeOwned + Send {}

#[derive(Deserialize)]
pub struct ParamsOided {
	pub oid: String,
}

#[derive(Deserialize)]
pub struct ParamsForOpen {
	pub path: String,
}

impl IntoParams for ParamsForOpen {}

#[derive(Deserialize)]
pub struct ParamsIded {
	pub id: i64,
}
impl IntoParams for ParamsIded {}
