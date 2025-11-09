use crate::{Error, Result};
use lib_core::{fire_event, Ctx};
use tauri_plugin_stronghold::stronghold::Stronghold;

pub fn save_to_vault(vault: &Stronghold, key: &str, value: &str) -> Result<()> {
	let store = vault.store();

	store
		.insert(key.as_bytes().to_vec(), value.as_bytes().to_vec(), None)
		.map_err(|err| Error::StrongholdStoreFail(err.to_string()))?;

	vault.save()?;
	Ok(())
}

pub fn get_from_vault(vault: &Stronghold, key: &str) -> Result<String> {
	let store = vault.store();

	let value = store
		.get(key.as_bytes())
		.map_err(|err| Error::StrongholdStoreFail(err.to_string()))?
		.ok_or(Error::StrongholdCredentialsNotFound)?;

	String::from_utf8(value).map_err(|_| Error::StrongholdCredentialsNotUtf8)
}
