// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub use error::{Error, Result};
mod error;

#[tokio::main]
async fn main() -> Result<()> {
	app_desktop_lib::run()
		.await
		.map_err(|_| "FATAL ERROR ->> FAILED TO RUN THE APPLICATION")?;
	Ok(())
}
