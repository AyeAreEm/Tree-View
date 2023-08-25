// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// really gotta work on the .unwrap()'s
// should do actual error handling
extern crate fs_extra;

use std::env;
// use std::path::PathBuf;
use std::process::Command;
use std::fs;
// use std::io;

use fs_extra::copy_items;
use fs_extra::dir;
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

fn get_extension(path: &str, is_file: bool) -> String {
    if is_file {
        let extension = match path.split(".").last() {
            Some(extension) => extension,
            None => "problem with getting data soz.",
        };
        return extension.to_string();
    }

    return "folder".to_string();
}

fn get_properties(directory: &str, filename: &str) -> Properties {
    let metadata_result = fs::metadata(directory);
    let metadata = match metadata_result {
        Ok(metadata) => metadata,
        Err(_) => return Properties { filename: "problem with getting data soz.".to_string(), location: "".to_string(), extension: "".to_string(), length: 0, is_directory: false, created: 0 },
    };

    let mut created: u64 = 0;

    if let Ok(time) = metadata.created() {
        created = time.duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs();
    }

    let properties = Properties {
        filename: filename.to_string(),
        location: directory.to_string(),
        extension: get_extension(directory, metadata.is_file()),
        length: metadata.len(),
        is_directory: metadata.is_dir(),
        created,
    };

    return properties;
}

#[tauri::command]
fn load_directory(directory: &str, user_ignores: Vec<String>) -> Vec<String> {
    let mut ignores = vec!["node_modules".to_string(), ".git".to_string(), "target".to_string(), ".DS_Store".to_string()];
    ignores.extend(user_ignores);

    let content: Vec<_> = WalkDir::new(directory)
                .follow_links(true)
                .into_iter()
                .filter_map(|f| f.ok())
                // refactor this, load filters from file (csv) maybe?
                .filter(|f| {
                    let path_display = f.path().display().to_string();
                    !ignores.iter().any(|ignore| path_display.contains(ignore))
                })
                .map(|f| f.path().display().to_string())
                .collect();

    if env::consts::OS == "windows" {
        let mut win_content: Vec<String> = Vec::new();
        for paths in content {
            win_content.push(paths.replace("\\", "/"));
        }

        return win_content
    }

    return content
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
        // figure out how to build with this
        tauri::WindowUrl::App("../../src/properties.html".into())
    ).build().unwrap();

    new_window.set_size(Size::Logical(tauri::LogicalSize { width: 350.0, height: 490.0 })).unwrap();
    new_window.set_minimizable(false).unwrap();
    new_window.set_title(format!("{filename} | Properties").as_str()).unwrap();
}

#[tauri::command]
fn get_properties_command(window: Window, directory: String, filename: String) {
    let properties = get_properties(&directory, &filename);
    window.emit("properties", &properties).unwrap();
}

fn open_on_windows(location_unsan: &String, application: &str) {
    let binding = location_unsan.replace("/", "\\");
    let location = binding.as_str();

    let index = if fs::metadata(location.clone()).unwrap().is_file() {location.rfind("\\").unwrap()} else {location.len()};

    match application {
        "" => {
            match open::that(location.clone()) {
                Ok(_) => (),
                Err(_) => {
                    open::that(location[0..index].to_string()).unwrap();
                }
            }
        },
        "term" => {
            Command::new("cmd")
                .arg("/K")
                .arg("cd")
                .arg("/d")
                .arg(format!("{}", &location[0..index]))
                .spawn()
                .unwrap();
        },
        "explorer" => {
            open::that(location[0..index].to_string()).unwrap();
        },
        _ => open::that(location).unwrap(),
    }

}

fn open_on_mac(location: &str, application: &str) {
    let index = if fs::metadata(location.clone()).unwrap().is_file() {location.rfind("/").unwrap()} else {location.len()};

    match application {
        "" => {
            match open::that(location.clone()) {
                Ok(_) => (),
                Err(_) => {
                    open::that(location[0..index].to_string()).unwrap();
                }
            }
        },
        "term" => {
            Command::new("open")
                .arg("-a")
                .arg("Terminal")
                .arg(format!("/{}", &location[0..index]))
                .spawn()
                .unwrap();
        },
        "explorer" => {
            open::that(location[0..index].to_string()).unwrap();
        }
        _ => open::that(location).unwrap(),

    }
}

#[tauri::command]
fn open_location(location: String, application: String) {
    if env::consts::OS == "windows" {
        open_on_windows(&location, &application);
    } else {
        open_on_mac(&location, &application);
    }
}

fn rename_handler(location: &String, new_location: String, filename: &str, slash: &str) -> (bool, String, i8) {
    let metadata_result = fs::metadata(location.clone());
    let metadata = match metadata_result {
        Ok(metadata) => metadata,
        Err(_) => return (false, "".to_string(), 1)
    };

    let path = format!("{}{}", slash, filename);
    let new = format!("{}{}{}", location.replace(&path, ""), slash, new_location);
    let new_san = match env::consts::OS {
        "windows" => new.replace("\\", "/"),
        _ => new.clone(),
    };

    match fs::rename(location, new.clone()) {
        Ok(_) => (metadata.is_dir(), new_san, 0),
        Err(_) => (false, "".to_string(), 1),
    }
}

#[tauri::command]
fn rename_location(location: String, new_location: String, filename: &str) -> (bool, String, i8) {
    if env::consts::OS == "windows" {
        let location_san = location.replace("/", "\\");

        return rename_handler(&location_san, new_location, filename, "\\")
    } else {
        return rename_handler(&location, new_location, filename, "/")
    }
}

#[tauri::command]
fn copy_paste(src: String, to: String) -> i8 {
    let options = dir::CopyOptions::new();

    match copy_items(&vec![src], to, &options) {
        Ok(_) => return 0,
        Err(e) => match e.kind {
            fs_extra::error::ErrorKind::AlreadyExists => {
                println!("already exists");
                return 0
            },
            _ => return 1,
        },
    }
}

#[tauri::command]
fn create_location(directory: String, mut filename: String) -> (String, i8) {
    filename = if !filename.ends_with("/") && !filename.contains(".") {
        format!("{}.txt", filename)
    } else {
        filename
    };

    let mut location = format!("{}/{}", directory, filename);

    if location.ends_with("/") {
        location.pop();
        return match fs::create_dir(location.clone()) {
            Ok(_) => (location.replace("\\", "/"), 0),
            Err(_) => ("".to_string(), 1), 
        }
    }

    match fs::File::create(location.clone()) {
        Ok(_) => (location.replace("\\", "/"), 0),
        Err(_) => ("".to_string(), 1),
    }
}

#[tauri::command]
fn remove_location(location: String) -> (bool, i8) {
    let metadata_result = fs::metadata(location.clone());
    let metadata = match metadata_result {
        Ok(metadata) => metadata,
        Err(_) => return (false, 1),
    };

    if metadata.is_dir() {
        match fs::remove_dir_all(location) {
            Ok(_) => (metadata.is_dir(), 0),
            Err(_) => (false, 1),
        }
    } else {
        match fs::remove_file(location) {
            Ok(_) => (metadata.is_dir(), 0),
            Err(_) => (false, 1),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_directory, make_properties_window, get_properties_command, open_location, rename_location, create_location, remove_location, copy_paste])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}