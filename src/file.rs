use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

use clap::Subcommand;

use crate::log;

#[derive(Subcommand, Debug)]
pub enum FileCommands {
    /// 列出文件/夹目录
    #[command(name = "list", alias = "ls")]
    List {
        /// 处理路径，默认为当前路径
        #[arg( num_args(0..=1))]
        target_path: Option<String>,
    },
    /// 打开文件
    #[command(name = "open", alias = "op")]
    Open {
        /// 处理路径，默认为当前路径
        #[arg( num_args(0..=1))]
        target_path: Option<String>,
    },
    /// 删除文件
    #[command(name = "delete", alias = "del")]
    Delete {
        /// 处理路径，默认为当前路径
        #[arg( num_args(0..=1))]
        target_path: Option<String>,
    },
    /// 重命名文件
    #[command(name = "rename", alias = "rn")]
    Rename {
        /// 被重命名的文件路径
        path: String,
        /// 重命名后的文件名
        target_path: String,
    },
}

fn resolve_path(path_opt: &Option<String>) -> Result<PathBuf, String> {
    let path_str = path_opt.as_deref().unwrap_or(".");
    let path = Path::new(path_str);

    path.canonicalize().map_err(|_| {
        let msg = format!("Invalid path: {}", path_str);
        log::error(&msg);
        msg
    })
}

fn show_file_info(entry: &DirEntry) {
    let file_name = entry.file_name();
    let file_name = file_name.to_string_lossy().into_owned();
    let file_type = entry.file_type().expect("Failed to get file type");
    let size = format!("{:.2}KB", entry.metadata().unwrap().len() as f64 / 1024.0);

    if file_type.is_file() {
        println!("{: <20} {: <20}", file_name, size);
    }

    if file_type.is_dir() {
        println!("{: <20} {: <20} ", format!("{}/", file_name), size);
    }
}

pub fn handle_command(file_commands: &FileCommands) {
    match file_commands {
        FileCommands::List { target_path } => {
            let target = match resolve_path(target_path) {
                Ok(path) => path,
                Err(_) => return,
            };

            if target_path.is_none() {
                println!("{: <20} {: <20} ", "Name", "Size");
            }

            let entries = std::fs::read_dir(&target).expect("Failed to read dir");

            let (dirs, files): (Vec<DirEntry>, Vec<DirEntry>) = entries
                .filter_map(Result::ok)
                .partition(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false));

            for entry in dirs {
                show_file_info(&entry);
            }
            for entry in files {
                show_file_info(&entry);
            }
        }
        FileCommands::Open { target_path } => {
            let target = match resolve_path(target_path) {
                Ok(path) => path,
                Err(_) => return,
            };

            if let Err(err) = opener::open(target) {
                log::error(&format!("Failed to open file: {}", err));
            }
        }
        FileCommands::Delete { target_path } => {
            let target = match resolve_path(target_path) {
                Ok(path) => path,
                Err(_) => return,
            };

            if let Err(err) = std::fs::remove_file(&target) {
                log::error(&format!("Failed to delete file: {}", err));
            } else {
                log::success("File deleted");
            }
        }
        FileCommands::Rename { target_path, path } => {
            if let Err(err) = std::fs::rename(path, target_path) {
                log::error(&format!("Failed to rename file: {}", err));
            } else {
                log::success("File renamed");
            }
        }
    }
}
