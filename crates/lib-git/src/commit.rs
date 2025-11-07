use gix::ObjectId;

#[derive(Debug, Clone)]
pub struct Commit {
	pub id: ObjectId,
	pub tree: ObjectId,
	pub message: String,
	pub author: String,
	pub time: i64,
}
