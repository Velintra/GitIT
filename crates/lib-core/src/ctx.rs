use crate::{HubEvent, error::Result};
use rpc_router::RpcResource;
use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Wry};

#[derive(Clone, RpcResource)]
pub struct Ctx {
	app_handle: AppHandle<Wry>,
	// TODO: Figure out where to put Repo
}

impl Ctx {
	pub fn arc_from_app(app: AppHandle<Wry>) -> Result<Arc<Ctx>> {
		Ok(Arc::new(Ctx::new(app)))
	}
	pub fn from_app(app: AppHandle<Wry>) -> Result<Ctx> {
		Ok(Ctx::new(app))
	}
}

impl Ctx {
	pub fn new(app_handle: AppHandle<Wry>) -> Self {
		Ctx { app_handle }
	}

	pub fn emit_hub_event<D: Serialize + Clone>(&self, hub_event: HubEvent<D>) {
		let _ = self.app_handle.emit("HubEvent", hub_event);
	}
}
