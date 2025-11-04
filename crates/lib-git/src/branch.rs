use git2::Oid;

#[derive(Debug, Clone)]
pub struct Branch {
	pub name: String,
	pub kind: String, // "Local" | "Remote"
	pub target: Option<Oid>,
}
