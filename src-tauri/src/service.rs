use simsearch::{SearchOptions, SimSearch};
use tauri::{GlobalShortcutManager, Manager};
use tauri::AppHandle;


pub fn init_engine() -> SimSearch<u32> {
    let origin_dict = &*crate::SEARCH_DICT.lock().unwrap();
    let mut choices = origin_dict.iter().map(|item| item.name.clone()).collect::<Vec<_>>();

    let mut engine: SimSearch<u32> = SimSearch::new_with(SearchOptions::new().threshold(-0.1));
    for (i, s) in choices.iter().enumerate() {
        engine.insert(i as u32, s.as_str());
    }
    engine
}

pub fn set_shortcut_event(ah: &AppHandle, shortcut: &str) {
  let window = ah.get_window("main").unwrap();
  let handle_clone = ah.clone();
  ah.global_shortcut_manager().unregister_all().unwrap();
  if !ah.global_shortcut_manager().is_registered(shortcut).unwrap() {
      ah.global_shortcut_manager().register(shortcut,  move || {
          crate::store::load_files(&handle_clone);
          window.show();
          window.set_focus();
          window.emit("set_focus", "");
      }).unwrap();
  }
}

pub fn fuzzy_search(string: &str, size: u32) -> Vec<crate::store::SearchItem> {
  let engine = &*crate::SEARCH_ENGINE.lock().unwrap();
  let search_dict = &*crate::SEARCH_DICT.lock().unwrap();
  let result = engine.search(string);

  result.iter().take(size as usize).map(|id| search_dict[*id as usize].clone()).collect()

}
