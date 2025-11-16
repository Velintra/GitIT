mod repo;
pub use repo::*;
use serde::Serialize;

use crate::{Ctx, HubEvent};

pub fn fire_event<D>(ctx: &Ctx, hub: &str, entity: &str, action: &str, data: D)
where
	D: Serialize + Clone,
{
	ctx.emit_hub_event(HubEvent {
		hub: hub.to_string(),
		topic: entity.to_string(),
		label: Some(action.to_string()),
		data: Some(data),
	});
}
