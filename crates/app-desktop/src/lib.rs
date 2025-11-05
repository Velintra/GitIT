// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//
//
mod error;
mod ipc;

pub use error::{Error, Result};

#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<()> {
	tauri::Builder::default()
		.plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![greet])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
	Ok(())
}
