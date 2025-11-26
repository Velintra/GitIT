use std::sync::Arc;

use lib_core::VaultManager;
use lib_event::{AppEvent, EventRx};
use tauri::{plugin::Plugin, AppHandle, Wry};

use crate::worker::VaultWorker;

pub struct VaultPlugin {
	vm: Arc<VaultManager>,
	rx: EventRx<AppEvent>,
}

impl VaultPlugin {
	pub fn new(vm: Arc<VaultManager>, rx: EventRx<AppEvent>) -> Self {
		Self { vm, rx }
	}
}

impl Plugin<Wry> for VaultPlugin {
	fn name(&self) -> &'static str {
		"vault_plugin"
	}

	fn initialization_script(&self) -> Option<String> {
		None
	}

	fn initialize(
		&mut self,
		app: &AppHandle<Wry>,
		_config: serde_json::Value,
	) -> Result<(), Box<dyn std::error::Error>> {
		let res = VaultWorker::start(self.rx.clone(), self.vm.clone(), app.clone())?;
		Ok(res)
	}
}
