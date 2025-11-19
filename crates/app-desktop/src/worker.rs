use std::sync::Arc;

use crate::{stronghold_utils::hash_blake3, Result};
use iota_stronghold::{KeyProvider, SnapshotPath, Stronghold};
use lib_core::{fire_event, Ctx, VaultManager};
use lib_event::{AppEvent, EventRx, VaultEvent};
use tauri::{AppHandle, Manager, Wry};

pub struct VaultWorker {
	rx: EventRx<AppEvent>,
	vm: Arc<VaultManager>,
	app_handle: AppHandle<Wry>,
}

impl VaultWorker {
	pub fn start(rx: EventRx<AppEvent>, vm: Arc<VaultManager>, app_handle: AppHandle<Wry>) -> Result<()> {
		let org_worker = VaultWorker { rx, vm, app_handle };

		tokio::spawn(async move {
			let res = org_worker.start_worker().await;
			res
		});
		Ok(())
	}

	async fn start_worker(&self) -> Result<()> {
		while let Ok(evt) = self.rx.recv().await {
			self.handle_event(evt)?
		}
		Ok(())
	}

	fn handle_event(&self, evt: AppEvent) -> Result<()> {
		match evt {
			AppEvent::Vault(vault_evt) => match vault_evt {
				VaultEvent::InitVault { password } => {
					let mut vault_path = self.app_handle.path().app_data_dir()?;
					std::fs::create_dir_all(&vault_path)?;
					vault_path.push("vault.gitit");

					let stronghold = Stronghold::default();

					let _client = stronghold.create_client(b"velintra")?;
					stronghold.write_client(b"velintra")?;

					let key = hash_blake3(password, "verynicesalt".into());
					let keyprovider = KeyProvider::try_from(key)?;
					let path = SnapshotPath::from_path(&vault_path);

					if path.exists() {
						stronghold.load_snapshot(&keyprovider, &path)?;
					} else {
						let _client = stronghold.create_client(b"velintra")?;
						stronghold.commit_with_keyprovider(&path, &keyprovider)?;
					}

					self.vm.set_vault(stronghold)?;

					let ctx = Ctx::from_app(self.app_handle.clone())?;
					fire_event(&ctx, "Handler", "vault", "init", true);
					Ok(())
				}
			},
		}
	}
}
