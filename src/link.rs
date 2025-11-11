use clap::Subcommand;
use regex::Regex;

use crate::{
    log,
    store::config::{get_store, set_store},
    util::resolve_path,
};
#[derive(Subcommand, Debug)]
pub enum LinkCommands {
    /// 创建链接
    Create {
        name: String,
        /// 处理路径，默认为当前路径
        #[arg( num_args(0..=1))]
        target_path: Option<String>,
    },
    /// 移除链接
    Remove { name: String },
    /// 显示链接
    Show { name: String },
}

pub fn handle_command(link_commands: &LinkCommands) {
    match link_commands {
        LinkCommands::Create { name, target_path } => {
            let target = match resolve_path(target_path) {
                Ok(path) => path,
                Err(_) => return,
            };

            let re: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
            if name.is_empty() || !re.is_match(name) {
                log::error("名称不合法");
                return;
            }

            let mut list = get_store("list");
            let is_insided = list.as_object().iter().any(|key| key.contains_key(name));

            if is_insided {
                log::error(&format!(
                    "链接已经存在: {} -> {}",
                    name,
                    list.get(name).unwrap().as_str().unwrap()
                ));
                return;
            }
            if !target.exists() {
                log::error(&format!("目标路径不存在: {}", target.display()));
                return;
            }

            list[name] = serde_json::Value::String(target.to_str().unwrap().to_string());
            set_store("list", list.clone());

            log::success(&format!("创建链接: {} -> {}", name, target.display()));
        }
        LinkCommands::Remove { name } => {
            let mut list = get_store("list");
            let is_insided = list.as_object().iter().any(|key| key.contains_key(name));
            if !is_insided {
                log::error(&format!("链接不存在: {}", name));
                return;
            }
            // 移除链接
            let obj = list.as_object_mut().unwrap();
            obj.remove(name);
            set_store("list", serde_json::Value::Object(obj.clone()));
            log::success(&format!("移除链接: {}", name));
        }
        LinkCommands::Show { name } => {
            let list = get_store("list");
            if let Some(target) = list.get(name) {
                log::info(&format!("{} -> {}", name, target.as_str().unwrap()));
                return;
            }
            log::error(&format!("链接不存在: {}", name));
        }
    }
}
