use tauri::Manager;
mod library;
use library::initiate::{get_meta, save_cover, save_page, setup_manga};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            let app_data_path = app
                .path()
                .app_data_dir()
                .unwrap_or_else(|_| std::env::current_dir().unwrap());

            let manager = library::initiate::MangaManager::new(app_data_path);

            app.manage(manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_meta,
            save_cover,
            save_page,
            setup_manga
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
