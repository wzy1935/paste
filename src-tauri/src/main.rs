#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate simsearch;

mod command;
mod inject;
mod service;
mod store;

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

use tauri::{GlobalShortcutManager, Manager};

lazy_static! {
    pub static ref SETTINGS: Mutex<store::Settings> = Mutex::new(store::Settings {
        version: "0.1".to_string()
    });
    pub static ref SEARCH_DICT: Mutex<Vec<store::SearchItem>> =
        Mutex::new(vec![store::SearchItem {
            name: "paste".to_string(),
            content: "welcome to paste!".to_string()
        }]);
    pub static ref SEARCH_ENGINE: Mutex<simsearch::SimSearch<u32>> =
        Mutex::new(service::init_engine());
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let ah = app.handle();
            store::load_files(&ah);

            let window = ah.get_window("main").unwrap();

            if !ah.global_shortcut_manager().is_registered("CommandOrControl+Space").unwrap() {
                ah.global_shortcut_manager().register("CommandOrControl+Space",  move || {
                    window.show();
                    window.set_focus();
                    window.emit("set_focus", "");
                });
            }
            ah.get_window("main").unwrap().hide();


            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::greet,
            command::inject_string,
            command::search_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
