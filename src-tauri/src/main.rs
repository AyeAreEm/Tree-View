// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use walkdir::WalkDir;

#[tauri::command]
fn load_directory(directory: &str) -> Vec<String> {
    let content: Vec<_> = WalkDir::new(directory)
                        .into_iter()
                        .filter_map(|f| f.ok())
                        .map(|f| f.path().to_owned())
                        .collect();

    let mut content_string: Vec<String> = Vec::new();

    for i in content {
        content_string.push(i.display().to_string());
    }

    return content_string
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
