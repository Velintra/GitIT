// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//
//
mod error;
mod handlers;
mod ipc;
use std::sync::Arc;
use tauri_plugin_stronghold::stronghold::Stronghold;
use uuid::Uuid;

use lib_core::{RepoManager, VaultManager};

pub use error::{Error, Result};

pub async fn run() -> Result<()> {
	let rm = RepoManager::default();
	let vm = Arc::new(VaultManager::default());

	let router = handlers::router_builder().append_resource(rm.clone()).build();
	tauri::Builder::default()
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_opener::init())
		.manage(router)
		.manage(vm)
		.invoke_handler(tauri::generate_handler![
			ipc::rpc_handler,
			ipc::save_credentials,
			ipc::get_credentials,
			ipc::init_vault
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
	Ok(())
}
