// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//
//
mod error;
mod handlers;
mod ipc;
use std::sync::Arc;

use lib_core::RepoManager;

pub use error::{Error, Result};

#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
	let rm = RepoManager::default();

	let router = handlers::router_builder().append_resource(rm.clone()).build();

	tauri::Builder::default()
		.plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
		.plugin(tauri_plugin_opener::init())
		.manage(router)
		.invoke_handler(tauri::generate_handler![ipc::rpc_handler])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
	Ok(())
}
