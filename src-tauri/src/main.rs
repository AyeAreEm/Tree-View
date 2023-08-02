// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// really gotta work on the .unwrap()'s
// should do actual error handling

use std::env;
use std::process::Command;
use std::fs;
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
fn load_directory(directory: &str) -> (Vec<String>, usize) {
    let content: Vec<_> = WalkDir::new(directory)
                .follow_links(true)
                .into_iter()
                .filter_map(|f| f.ok())
                // refactor this, load filters from file (csv) maybe?
                .filter(|f| !f.path().display().to_string().contains("node_modules") && !f.path().display().to_string().contains(".git") && !f.path().display().to_string().contains("target") && !f.path().display().to_string().contains(".DS_Store"))
                .map(|f| f.path().display().to_string())
                .collect();

    let real_size = content.len(); 

    return (content, real_size)
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

    new_window.set_size(Size::Logical(tauri::LogicalSize { width: 350.0, height: 460.0 })).unwrap();
    new_window.set_minimizable(false).unwrap();
    new_window.set_title(format!("{filename} | Properties").as_str()).unwrap();
}

#[tauri::command]
fn get_properties_command(window: Window, directory: String, filename: String) {
    let properties = get_properties(&directory, &filename);
    window.emit("properties", &properties).unwrap();
}

fn open_on_windows(location: &str, application: &str) {
    let result = location.replace("/", "\\");
    let index = if fs::metadata(result.clone()).unwrap().is_file() {result.rfind("\\").unwrap()} else {result.len()};

    match application {
        "" => {
            match open::that(result.clone()) {
                Ok(_) => (),
                Err(_) => {
                    open::that(result[0..index].to_string()).unwrap();
                }
            }
        },
        "term" => {
            Command::new("cmd")
                .arg("/K")
                .arg("cd")
                .arg("/d")
                .arg(format!("{}", &result[0..index]))
                .spawn()
                .unwrap();
        },
        "explorer" => {
            open::that(result[0..index].to_string()).unwrap();
        },
        _ => open::that(result).unwrap(),
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

#[tauri::command]
fn create_location(location: String) -> i8 {
    if location.ends_with("/") {
        match fs::create_dir(location) {
            Ok(_) => return 0,
            Err(_) => {
                println!("couldn't create entity");
                return 1
            }, 
        }
    }

    match fs::File::create(location) {
        Ok(_) => return 0,
        Err(_) => {
            println!("couldn't create entity");
            return 1
        },
    }
}

#[tauri::command]
fn remove_location(location: String, is_dir: bool) -> i8 {
    if is_dir {
        match fs::remove_dir_all(location) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    } else {
        match fs::remove_file(location) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_directory, make_properties_window, get_properties_command, open_location, create_location, remove_location])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
