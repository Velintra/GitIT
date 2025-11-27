use gix::ObjectId;
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, Serialize)]
pub struct Branch {
	pub name: String,
	pub kind: String, // "Local" | "Remote"
	#[serde_as(as = "serde_with::DisplayFromStr")]
	pub target: ObjectId,
}
