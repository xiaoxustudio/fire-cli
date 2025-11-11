use crate::{log, store::config::get_store};

pub fn handle_coommand(target: &String) {
    if target.starts_with("http://") || target.starts_with("https://") {
        log::info(&format!("Opening URL: {}", target));
        opener::open(target).expect("Failed to open URL");
        return;
    }
    let list = get_store("list");
    let is_insided = list.as_object().iter().any(|key| key.contains_key(target));
    if is_insided {
        if let Some(path_value) = list.get(target) {
            if let Some(path_str) = path_value.as_str() {
                open::that(path_str).expect("Failed to open file");
                return;
            }
        }
        return;
    }
    log::error(&format!("无效的关键字: {}", target));
}
