use gix::ObjectId;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Branch {
	pub name: String,
	pub kind: String, // "Local" | "Remote"
	pub target: ObjectId,
}
