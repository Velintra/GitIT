use crate::{Error, Result};
use iota_stronghold::Store;
use lib_core::{fire_event, Ctx};
use tauri_plugin_stronghold::stronghold::Stronghold;

pub fn save_to_vault(vault: &Store, key: &str, value: &str) -> Result<()> {
	vault
		.insert(key.as_bytes().to_vec(), value.as_bytes().to_vec(), None)
		.map_err(|err| Error::StrongholdStoreFail(err.to_string()))?;

	Ok(())
}

pub fn save_multiple_to_vault(vault: &Store, entries: &[(impl AsRef<[u8]>, impl AsRef<[u8]>)]) -> Result<()> {
	for (k, v) in entries {
		vault
			.insert(k.as_ref().to_vec(), v.as_ref().to_vec(), None)
			.map_err(|err| Error::StrongholdStoreFail(err.to_string()))?;
	}

	Ok(())
}

pub fn get_from_vault(store: &Store, key: &str) -> Result<String> {
	let value = store
		.get(key.as_bytes())
		.map_err(|err| Error::StrongholdStoreFail(err.to_string()))?
		.ok_or(Error::StrongholdCredentialsNotFound)?;

	String::from_utf8(value).map_err(|_| Error::StrongholdCredentialsNotUtf8)
}
