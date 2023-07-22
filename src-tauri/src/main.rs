// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// really gotta work on the .unwrap()'s
// should do actual error handling

use std::env;
use std::process::Command;
use tauri::Size;
use tauri::Window;
use walkdir::WalkDir;
use open;
use rand::Rng;
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
                .follow_links(true)
                .into_iter()
                .filter_map(|f| f.ok())
                .filter(|f| !f.path().display().to_string().contains("node_modules") && !f.path().display().to_string().contains(".git") && !f.path().display().to_string().contains("target"))
                .map(|f| f.path().display().to_string().to_owned())
                .collect();

    let mut content_string: Vec<String> = Vec::new();

    for i in content {
        content_string.push(i);
    }

    if content_string.len() > 50 {
        let content_msg: Vec<String> = vec!["this directory too big lol soz.".to_string()];
        return content_msg;
    }

    return content_string
}

#[tauri::command]
async fn make_properties_window(handle: tauri::AppHandle, filename: String) {
    // this is needed because tauri doesn't like having multiple windows with the same label
    // but for some reason, i can't just use "filename"
    // so random number it is

    let label = rand::thread_rng().gen_range(0..10000);

    let new_window = tauri::WindowBuilder::new(
        &handle,
        format!("{}", label),
        tauri::WindowUrl::App("../../src/properties.html".into())
    ).build().unwrap();

    new_window.set_size(Size::Logical(tauri::LogicalSize { width: 350.0, height: 450.0 })).unwrap();
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
    let result = if env::consts::OS == "windows" { location.replace("/", "\\") } else { location };

    if application == "" {
        open::that(result).unwrap();
        return
    }

    match open::with(result.to_owned(), application) {
        Ok(_) => (),
        Err(_) => {
            if env::consts::OS == "windows" {
                Command::new("cmd")
                    .arg("/K")
                    .arg("cd")
                    .arg("/d")
                    .arg(format!("{}", result))
                    .spawn()
                    .unwrap();
            } else {
                Command::new("open")
                    .arg("-a")
                    .arg("Terminal")
                    .arg(format!("/{}", result))
                    .spawn()
                    .unwrap();
            }
        }, 
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_directory, make_properties_window, get_properties_command, open_location])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
