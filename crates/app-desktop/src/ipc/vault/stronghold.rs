use crate::{stronghold_utils, Result};
use lib_core::{fire_event, Ctx, ModelManager, VaultManager};
use lib_event::VaultEvent;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, State, Wry};

const USERNAME_KEY: &str = "git_username";
const PWD_KEY: &str = "git_password";

#[derive(Debug, Deserialize, Serialize)]
pub struct ParamsForVaultInsert {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Credentials {
	pub username: String,
	pub password: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct InitVaultParams {
	pub password: String,
}

#[tauri::command]
pub async fn init_vault(mm: State<'_, Arc<ModelManager>>, params: InitVaultParams) -> Result<()> {
	let tx = mm.app_tx().evt_tx();

	let evt = VaultEvent::InitVault {
		password: params.password,
	};

	tx.send(evt.into()).await?;

	Ok(())
}

#[tauri::command]
pub fn save_credentials(
	app_handle: AppHandle<Wry>,
	vm: State<'_, Arc<VaultManager>>,
	params: ParamsForVaultInsert,
) -> Result<()> {
	let ctx = Ctx::from_app(app_handle)?;
	let client = vm.get_or_create_client(b"velintra")?;

	stronghold_utils::save_multiple_to_vault(
		&client.store(),
		&[(USERNAME_KEY, &params.username), (PWD_KEY, &params.password)],
	)?;

	fire_event(&ctx, "Handler", "creds", "save", true);

	Ok(())
}

#[tauri::command]
pub fn get_credentials(vm: State<'_, Arc<VaultManager>>) -> Result<Credentials> {
	let client = vm.get_or_create_client(b"velintra")?;

	let username = stronghold_utils::get_from_vault(&client.store(), USERNAME_KEY)?;
	let password = stronghold_utils::get_from_vault(&client.store(), PWD_KEY)?;

	Ok(Credentials { username, password })
}
