// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Size;
use tauri::Window;
use walkdir::WalkDir;
use open;
use serde::{Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
struct Properties {
    filename: String,
    location: String,
    extension: String,
    length: u64,
    is_directory: bool,
    created: u64,
}

fn get_extension(path: &str) -> Option<&str> {
    path.split('.').last()
}

fn get_properties(directory: &str, filename: &str) -> Properties {
    let metadata = std::fs::metadata(directory).unwrap();
    let mut created: u64 = 0;

    if let Ok(time) = metadata.created() {
        created = time.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs();
    }

    let properties = Properties {
        filename: filename.to_string(),
        location: directory.to_string(),
        extension: if metadata.is_file() {get_extension(directory).unwrap().to_string()} else {"Folder".to_string()},
        length: metadata.len(),
        is_directory: metadata.is_dir(),
        created,
    };

    return properties;
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


#[tauri::command]
async fn make_properties_window(handle: tauri::AppHandle, filename: String) {
    let new_window = tauri::WindowBuilder::new(
        &handle,
        "Properties",
        tauri::WindowUrl::App("../../src/properties.html".into())
    ).build().unwrap();

    new_window.set_size(Size::Physical(tauri::PhysicalSize { width: 350, height: 450 })).unwrap();
    new_window.set_resizable(false).unwrap();
    new_window.set_minimizable(false).unwrap();
    new_window.set_title(format!("{filename} | Properties").as_str()).unwrap();
}

#[tauri::command]
fn get_properties_command(window: Window, directory: String, filename: String) {
    let properties = get_properties(&directory, &filename);
    window.emit("properties", &properties).unwrap();
}

#[tauri::command]
fn open_location(location: String, application: String) {
    let result = location.replace("/", "\\");

    if application == "" {
        open::that(result).unwrap();
        return
    } else if application == "nvim" {
        open::with(result, application).unwrap();
        return
    }

    match open::with(result.to_owned(), application) {
        Ok(_) => (),
        Err(_) => {
            open::with(result, "cmd").unwrap();
        }, 
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_directory, make_properties_window, get_properties_command, open_location])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
