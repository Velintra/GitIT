use serde::Serialize;

#[derive(Serialize)]
pub struct DataIpcResult<T>
where
	T: Serialize,
{
	data: T,
}

impl<T> From<T> for DataIpcResult<T>
where
	T: Serialize,
{
	fn from(val: T) -> Self {
		Self { data: val }
	}
}
