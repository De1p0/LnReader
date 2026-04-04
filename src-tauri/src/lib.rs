use tauri::Manager;
mod library;
use library::initiate::{
    add_chapter, add_manga, get_manga, list_manga, remove_manga, save_cover, set_chapter_read,
    MangaManager,
};

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

            let manager = MangaManager::new(app_data_path);
            app.manage(manager);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            add_manga,
            add_chapter,
            set_chapter_read,
            get_manga,
            list_manga,
            remove_manga,
            save_cover
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
