use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use tauri::State;
use tauri_plugin_stronghold::stronghold::Stronghold;

const USERNAME_KEY: &str = "git_username";
const TOKEN_KEY: &str = "git_token";

#[derive(Debug, Deserialize, Serialize)]
pub struct ParamsForVaultInsert {
	pub username: String,
	pub token: String,
}

#[derive(Debug, Serialize)]
pub struct Credentials {
	pub username: String,
	pub token: String,
}

#[tauri::command]
pub fn save_credentials(vault: State<'_, Stronghold>, params: ParamsForVaultInsert) -> Result<()> {
	let ParamsForVaultInsert { username, token } = params;
	let store = vault.store();

	store
		.insert(USERNAME_KEY.as_bytes().to_vec(), username.into_bytes(), None)
		.map_err(|err| Error::StrongholdStoreFail(err.to_string()))?;

	store
		.insert(TOKEN_KEY.as_bytes().to_vec(), token.into_bytes(), None)
		.map_err(|err| Error::StrongholdStoreFail(err.to_string()))?;

	vault.save()?;

	Ok(())
}

#[tauri::command]
pub fn get_credentials(vault: State<'_, Stronghold>) -> Result<Credentials> {
	let store = vault.store();

	let username = store
		.get(USERNAME_KEY.as_bytes())
		.map_err(|err| Error::StrongholdStoreFail(err.to_string()))?
		.ok_or(Error::StrongholdCredentialsNotFound)?;

	let token = store
		.get(TOKEN_KEY.as_bytes())
		.map_err(|err| Error::StrongholdStoreFail(err.to_string()))?
		.ok_or(Error::StrongholdCredentialsNotFound)?;

	let username = String::from_utf8(username).map_err(|_| Error::StrongholdCredentialsNotUtf8)?;
	let token = String::from_utf8(token).map_err(|_| Error::StrongholdCredentialsNotUtf8)?;

	Ok(Credentials { username, token })
}
