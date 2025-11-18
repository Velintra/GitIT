use crate::Result;
use lib_event::{AppEvent, EventRx, TauriEvent};

pub struct VaultWorker {
	rx: EventRx<AppEvent>,
}

impl VaultWorker {
	pub fn start(rx: EventRx<AppEvent>) -> Result<()> {
		let org_worker = VaultWorker { rx };

		tokio::spawn(async move {
			let res = org_worker.start_worker().await;
			res
		});
		Ok(())
	}

	async fn start_worker(&self) -> Result<()> {
		while let Ok(evt) = self.rx.recv().await {
			match evt {
				AppEvent::Tauri(evt) => match evt {
					TauriEvent::InitVault { password } => {}
				},
			}
		}
		Ok(())
	}
}
