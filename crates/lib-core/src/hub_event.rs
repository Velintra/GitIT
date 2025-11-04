use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct HubEvent<D: Serialize + Clone> {
	pub hub: String,
	pub topic: String,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<D>,
}
