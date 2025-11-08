// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//
//
mod error;
mod handlers;
mod ipc;
use std::sync::Arc;
use uuid::Uuid;

use lib_core::RepoManager;

pub use error::{Error, Result};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
	let rm = RepoManager::default();

	let router = handlers::router_builder().append_resource(rm.clone()).build();

	tauri::Builder::default()
		.plugin(tauri_plugin_dialog::init())
		.plugin(
			tauri_plugin_stronghold::Builder::new(|pass| {
				let mut hasher = blake3::Hasher::new();
				hasher.update(pass.as_bytes());
				let salt = Uuid::new_v4();
				hasher.update(salt.as_bytes());
				hasher.finalize().as_bytes().to_vec()
			})
			.build(),
		)
		.plugin(tauri_plugin_opener::init())
		.manage(router)
		.invoke_handler(tauri::generate_handler![
			ipc::rpc_handler,
			ipc::save_credentials,
			ipc::get_credentials
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
	Ok(())
}
