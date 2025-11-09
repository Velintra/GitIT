use crate::{ipc::vault::support, Error, Result};
use lib_core::{fire_event, Ctx, VaultManager};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State, Wry};
use tauri_plugin_stronghold::stronghold::Stronghold;
use uuid::Uuid;

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
pub fn init_vault(app_handle: AppHandle<Wry>, vm: State<'_, VaultManager>, params: InitVaultParams) -> Result<()> {
	let mut vault_path = app_handle.path().app_data_dir()?;
	vault_path.push("vault.hold");
	let ctx = Ctx::from_app(app_handle)?;

	let pwd_hash = {
		let mut hasher = blake3::Hasher::new();
		hasher.update(params.password.as_bytes());
		let salt = Uuid::new_v4();
		hasher.update(salt.as_bytes());
		hasher.finalize().as_bytes().to_vec()
	};

	let stronghold = Stronghold::new(vault_path, pwd_hash)?;
	vm.set_vault(stronghold)?;
	fire_event(&ctx, "Handler", "vault", "init", true);

	Ok(())
}

#[tauri::command]
pub fn save_credentials(
	app_handle: AppHandle<Wry>,
	vm: State<'_, VaultManager>,
	params: ParamsForVaultInsert,
) -> Result<()> {
	let ctx = Ctx::from_app(app_handle)?;
	let guard = vm.get_vault()?;
	let vault = guard.get()?;
	support::save_to_vault(&vault, USERNAME_KEY, &params.username)?;
	support::save_to_vault(&vault, PWD_KEY, &params.password)?;
	fire_event(&ctx, "Handler", "creds", "save", true);
	Ok(())
}

#[tauri::command]
pub fn get_credentials(vm: State<'_, VaultManager>) -> Result<Credentials> {
	let guard = vm.get_vault()?;
	let vault = guard.get()?;
	let username = support::get_from_vault(&vault, USERNAME_KEY)?;
	let password = support::get_from_vault(&vault, PWD_KEY)?;
	Ok(Credentials { username, password })
}
