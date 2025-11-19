mod repo_manager;
mod vault_manager;
use lib_event::AppTx;
pub use repo_manager::*;
pub use vault_manager::*;

#[derive(Clone)]
pub struct ModelManager {
	tx: AppTx,
}

impl ModelManager {
	pub fn new(tx: AppTx) -> Self {
		ModelManager { tx }
	}

	pub fn with_txw(&self, tx: AppTx) -> Self {
		Self { tx }
	}

	pub fn app_tx(&self) -> &AppTx {
		&self.tx
	}
}
