extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use tauri::AppHandle;
use tauri::Manager;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SearchItem {
    pub name: String,
    pub content: String,
}

fn init_folder(ah: &AppHandle) {
    let data_dir = ah.path_resolver().app_dir().unwrap();
    let data_dir = data_dir.to_str().unwrap();
    let data_path = std::path::Path::new(&data_dir);
    if !data_path.exists() {
        std::fs::create_dir(&data_path).unwrap();
    }
}

// TODO: bad coding here. Consider Refactoring.

fn init_files(ah: &AppHandle) {
    init_folder(&ah);
    let data_dir = ah.path_resolver().app_dir().unwrap();
    let data_dir = data_dir.to_str().unwrap();
    let setting_path = std::path::Path::new(data_dir.clone()).join("settings.json");
    let search_path = std::path::Path::new(data_dir.clone()).join("search.json");
    if !setting_path.as_path().exists() {
        let mut setting_file = File::create(&setting_path.as_path()).unwrap();
        setting_file.write(
            json!(*crate::SETTINGS.lock().unwrap())
                .to_string()
                .as_bytes(),
        );
    }
    if !search_path.as_path().exists() {
        let mut search_file = File::create(&search_path.as_path()).unwrap();
        search_file.write(
            json!(*crate::SEARCH_DICT.lock().unwrap())
                .to_string()
                .as_bytes(),
        );
    }
}

pub fn load_files(ah: &AppHandle) {
    init_files(&ah);
    let data_dir = ah.path_resolver().app_dir().unwrap();
    let data_dir = data_dir.to_str().unwrap();
    let setting_path = std::path::Path::new(data_dir.clone()).join("settings.json");
    let search_path = std::path::Path::new(data_dir.clone()).join("search.json");

    let mut setting_file = File::open(setting_path.as_path()).unwrap();
    let mut search_file = File::open(search_path.as_path()).unwrap();

    let mut setting_content = String::new();
    let mut search_content = String::new();

    setting_file.read_to_string(&mut setting_content);
    search_file.read_to_string(&mut search_content);

    *crate::SETTINGS.lock().unwrap() = serde_json::from_str(setting_content.as_str()).unwrap();
    *crate::SEARCH_DICT.lock().unwrap() = serde_json::from_str(search_content.as_str()).unwrap();
}

pub fn save_files(ah: &AppHandle) {
    init_folder(&ah);
    let data_dir = ah.path_resolver().app_dir().unwrap();
    let data_dir = data_dir.to_str().unwrap();
    let setting_path = std::path::Path::new(data_dir.clone()).join("settings.json");
    let search_path = std::path::Path::new(data_dir.clone()).join("search.json");

    let mut setting_file = File::create(&setting_path.as_path())
        .unwrap_or(File::open(&setting_path.as_path()).unwrap());
    let mut search_file =
        File::create(&search_path.as_path()).unwrap_or(File::open(&search_path.as_path()).unwrap());

    setting_file.write(
        json!(*crate::SETTINGS.lock().unwrap())
            .to_string()
            .as_bytes(),
    );
    search_file.write(
        json!(*crate::SEARCH_DICT.lock().unwrap())
            .to_string()
            .as_bytes(),
    );
}
