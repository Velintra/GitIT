// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//
//
mod error;
mod handlers;
mod ipc;
mod plugins;
mod stronghold_utils;
mod worker;
use std::sync::Arc;

use lib_core::{ModelManager, RepoManager, VaultManager};

pub use error::{Error, Result};
use lib_event::{new_trx_pair, AppEvent, AppTx};

use crate::plugins::VaultPlugin;

pub async fn run() -> Result<()> {
	let rm = RepoManager::default();
	let vm = Arc::new(VaultManager::default());

	let router = handlers::router_builder().append_resource(rm.clone()).build();

	let (tx, rx) = new_trx_pair::<AppEvent>();
	let app_tx = AppTx::new(tx);

	let mm = Arc::new(ModelManager::new(app_tx));

	let vault_plugin = VaultPlugin::new(vm.clone(), rx);

	tauri::Builder::default()
		.plugin(tauri_plugin_dialog::init())
		.plugin(tauri_plugin_opener::init())
		.plugin(vault_plugin)
		.manage(router)
		.manage(vm.clone())
		.manage(mm)
		.invoke_handler(tauri::generate_handler![
			ipc::rpc_handler,
			ipc::save_credentials,
			ipc::get_credentials,
			ipc::init_vault
		])
		.run(tauri::generate_context!())?;

	Ok(())
}
