mod repo;

pub use repo::*;
use serde::Serialize;

use crate::{Ctx, HubEvent};

pub fn fire_model_event<D>(ctx: &Ctx, entity: &str, action: &str, data: D)
where
	D: Serialize + Clone,
{
	ctx.emit_hub_event(HubEvent {
		hub: "Model".to_string(),
		topic: entity.to_string(),
		label: Some(action.to_string()),
		data: Some(data),
	});
}
