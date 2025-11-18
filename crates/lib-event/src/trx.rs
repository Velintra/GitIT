use crate::{error::Result, AppEvent};
use flume::{Receiver, Sender};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct AppTx {
	evt_tx: EventTx<AppEvent>,
}

impl AppTx {
	pub fn new(evt_tx: EventTx<AppEvent>) -> Self {
		Self { evt_tx }
	}

	pub fn evt_tx(&self) -> &EventTx<AppEvent> {
		&self.evt_tx
	}
}

#[derive(Clone)]
pub struct EventTx<T> {
	tx: Sender<T>,
}

impl<T> EventTx<T>
where
	T: Serialize + DeserializeOwned,
{
	pub async fn send(&self, item: T) -> Result<()> {
		match self.tx.send_async(item.into()).await {
			Ok(_) => Ok(()),
			Err(ex) => Err(ex.into()),
		}
	}
}

impl<T> From<Sender<T>> for EventTx<T> {
	fn from(tx: Sender<T>) -> Self {
		Self { tx }
	}
}

pub struct EventRx<T> {
	rx: Receiver<T>,
}

impl<T> EventRx<T>
where
	T: Serialize + DeserializeOwned,
{
	pub async fn recv(&self) -> Result<T> {
		let res = self.rx.recv_async().await?;
		Ok(res)
	}
}

pub fn new_trx_pair<T>() -> (EventTx<T>, EventRx<T>) {
	let (tx, rx) = flume::unbounded::<T>();

	let evt_tx = EventTx { tx };

	let evt_rx = EventRx { rx };

	(evt_tx, evt_rx)
}
