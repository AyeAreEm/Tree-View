// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Size;
use tauri::Window;
use walkdir::WalkDir;
use serde::{Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
struct Properties {
    extension: String,
    length: u64,
    is_directory: bool,
    is_file: bool,
    created: u64,
}

fn get_extension(path: &str) -> Option<&str> {
    path.split('.').last()
}

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

fn get_properties() -> Properties {
    let metadata = std::fs::metadata("C:/Users/agaye/Desktop/testing/first-test.txt").unwrap();
    let mut created: u64 = 0;

    if let Ok(time) = metadata.created() {
        created = time.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs();
    }

    let properties = Properties {
        extension: get_extension("C:/Users/agaye/Desktop/testing/first-test.txt").unwrap().to_string(),
        length: metadata.len(),
        is_directory: metadata.is_dir(),
        is_file: metadata.is_file(),
        created,
    };

    return properties;
}

#[tauri::command]
async fn make_properties_window(handle: tauri::AppHandle) {
    let new_window = tauri::WindowBuilder::new(
        &handle,
        "Properties",
        tauri::WindowUrl::App("../../src/properties.html".into())
    ).build().unwrap();

    new_window.set_size(Size::Physical(tauri::PhysicalSize { width: 350, height: 450 })).unwrap();
    new_window.set_resizable(false).unwrap();
}

#[tauri::command]
fn test(window: Window) {
    let properties = get_properties();
    window.emit("properties", &properties).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_directory, make_properties_window, test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
