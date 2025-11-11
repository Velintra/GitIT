use crate::{ipc::vault::support, Error, Result};
use iota_stronghold::{KeyProvider, SnapshotPath, Stronghold};
use lib_core::{fire_event, Ctx, VaultManager};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Manager, State, Wry};
use uuid::Uuid;
use zeroize::Zeroizing;

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
pub fn init_vault(app_handle: AppHandle<Wry>, vm: State<'_, Arc<VaultManager>>, params: InitVaultParams) -> Result<()> {
	let mut vault_path = app_handle.path().app_data_dir()?;
	std::fs::create_dir_all(&vault_path)?;
	vault_path.push("vault.hold");
	let ctx = Ctx::from_app(app_handle)?;

	let mut hasher = blake3::Hasher::new();
	hasher.update(params.password.as_bytes());
	hasher.update(b"very-nice-saltyy-salt");
	let key_bytes_vec = hasher.finalize().as_bytes().to_vec();
	let key_bytes = Zeroizing::new(key_bytes_vec);

	let keyprovider = KeyProvider::try_from(key_bytes)?;
	let stronghold = Stronghold::default();
	let path = SnapshotPath::from_path(&vault_path);

	if path.exists() {
		stronghold.load_snapshot(&keyprovider, &path)?;
	} else {
		let _client = stronghold.create_client(b"velintra")?;
		stronghold.write_client(b"velintra")?;
		stronghold.commit_with_keyprovider(&path, &keyprovider)?; // FIXME: Crashes the app
	}

	vm.set_vault(stronghold)?;

	fire_event(&ctx, "Handler", "vault", "init", true);

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

	support::save_multiple_to_vault(
		&client.store(),
		&[(USERNAME_KEY, &params.username), (PWD_KEY, &params.password)],
	)?;

	fire_event(&ctx, "Handler", "creds", "save", true);

	Ok(())
}

#[tauri::command]
pub fn get_credentials(vm: State<'_, Arc<VaultManager>>) -> Result<Credentials> {
	let client = vm.get_or_create_client(b"velintra")?;

	let username = support::get_from_vault(&client.store(), USERNAME_KEY)?;
	let password = support::get_from_vault(&client.store(), PWD_KEY)?;

	Ok(Credentials { username, password })
}
