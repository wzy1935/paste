use tauri::command;
use tauri::Manager;

#[command]
pub fn greet(name: &str) -> String {
    println!("{}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
pub fn inject_string(app: tauri::AppHandle, string: &str) {
    app.get_window("main").unwrap().hide().unwrap();
    if string.len() > 0 {
        crate::inject::inject_string(string.to_string());
    }
}

#[command]
pub fn search_content(content: &str) -> Vec<crate::store::SearchItem> {
    if content.len() > 0 {
        crate::service::fuzzy_search(content, 5)
    } else {
      vec![]
    }
}
