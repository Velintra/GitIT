use crate::error::{Error, Result};
use rpc_router::RpcResource;
use std::sync::{Arc, Mutex, MutexGuard};
use tauri_plugin_stronghold::stronghold::Stronghold;

#[derive(Default, Clone, RpcResource)]
pub struct VaultManager {
	vault: Arc<Mutex<Option<Stronghold>>>,
}

pub struct VaultGuard<'a> {
	guard: MutexGuard<'a, Option<Stronghold>>,
}

impl<'a> VaultGuard<'a> {
	pub fn get(&self) -> Result<&Stronghold> {
		self.guard.as_ref().ok_or(Error::VaultNotInitialized)
	}
}

impl VaultManager {
	pub fn get_vault(&self) -> Result<VaultGuard<'_>> {
		let guard = self.vault.lock()?;
		Ok(VaultGuard { guard })
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
