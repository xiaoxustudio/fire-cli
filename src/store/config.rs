use std::{
    fs::{self, remove_file},
    path::PathBuf,
};

use crate::util::write_file;

use std::sync::LazyLock;

static DEFAULT_CONFIG: LazyLock<serde_json::Value> = LazyLock::new(|| {
    serde_json::json!({
        "list": {},
    })
});

fn get_config_dir() -> PathBuf {
    let mut user_local = dirs::config_dir().unwrap();
    user_local.push("fire-cli/config.json");
    user_local
}

fn get_config_content() -> String {
    let user_local = get_config_dir();

    if fs::exists(user_local.clone()).is_err() || fs::exists(user_local.clone()).unwrap() == false {
        write_file(
            user_local.clone(),
            serde_json::json!(DEFAULT_CONFIG.clone()).to_string(),
        )
        .unwrap();
    }

    match fs::read_to_string(user_local) {
        Ok(v) => v,
        Err(_) => "".to_string(),
    }
}

fn get_config() -> serde_json::Value {
    let user_local = get_config_dir();
    let content = get_config_content();
    let content = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => {
            remove_file(user_local).unwrap();
            get_config();
            DEFAULT_CONFIG.clone()
        }
    };
    content
}

pub fn get_store(name: &str) -> serde_json::Value {
    let content = get_config().clone();
    content[name].clone()
}

pub fn set_store(name: &str, value: serde_json::Value) {
    let mut content = get_config();
    content[name] = value;
    write_file(get_config_dir(), content.to_string()).unwrap();
}
