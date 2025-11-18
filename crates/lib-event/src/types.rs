use derive_more::From;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Debug, Clone, From, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppEvent {
	#[from]
	Tauri(TauriEvent),
}
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TauriEvent {}
