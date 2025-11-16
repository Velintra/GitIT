use gix::ObjectId;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Commit {
	pub id: ObjectId,
	pub tree: ObjectId,
	pub message: String,
	pub author: String,
	pub time: i64,
}
