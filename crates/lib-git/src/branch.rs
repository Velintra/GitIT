use gix::ObjectId;

#[derive(Debug, Clone)]
pub struct Branch {
	pub name: String,
	pub kind: String, // "Local" | "Remote"
	pub target: ObjectId,
}
