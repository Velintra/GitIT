use git2::Oid;

pub struct Commit {
	pub id: Oid,
	pub tree: Oid,
	pub message: String,
	pub author: String,
	pub time: i64,
}
