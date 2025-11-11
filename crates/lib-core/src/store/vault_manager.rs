use crate::error::{Error, Result};
use iota_stronghold::{Client, Stronghold};
use rpc_router::RpcResource;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Default, Clone, RpcResource)]
pub struct VaultManager {
	vault: Arc<Mutex<Option<Stronghold>>>,
}

impl VaultManager {
	pub fn get_vault(&self) -> Result<MutexGuard<'_, Option<Stronghold>>> {
		let guard = self.vault.lock()?;
		Ok(guard)
	}

	pub fn is_initialized(&self) -> bool {
		self.vault.lock().map(|guard| guard.is_some()).unwrap_or(false)
	}

	pub fn get_or_create_client(&self, path: &[u8]) -> Result<Client> {
		let guard = self.vault.lock()?;
		let stronghold = guard.as_ref().ok_or(Error::VaultNotInitialized)?;

		match stronghold.get_client(path) {
			Ok(client) => Ok(client),
			Err(_) => stronghold.create_client(path).map_err(Error::from),
		}
	}

	pub fn set_vault(&self, stronghold: Stronghold) -> Result<()> {
		let mut guard = self.vault.lock()?;
		*guard = Some(stronghold);
		Ok(())
	}

	pub fn clear(&self) -> Result<()> {
		let mut guard = self.vault.lock()?;
		*guard = None;
		Ok(())
	}
}
