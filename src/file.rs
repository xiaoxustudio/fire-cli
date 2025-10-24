use std::{
    fs::{self, DirEntry},
    io::{self, BufRead, Write},
};

use clap::Subcommand;

use crate::{log, util::resolve_path};

#[derive(Subcommand, Debug)]
pub enum FileCommands {
    /// 列出文件/夹目录
    #[command(name = "list", alias = "ls")]
    List {
        /// 处理路径，默认为当前路径
        #[arg(num_args(0..=1))]
        target_path: Option<String>,
    },
    /// 打开文件
    #[command(name = "open", alias = "op")]
    Open {
        /// 处理路径，默认为当前路径
        #[arg(num_args(0..=1))]
        target_path: Option<String>,
    },
    /// 删除文件/目录
    #[command(name = "delete", alias = "del")]
    Delete {
        /// 处理路径，默认为当前路径
        #[arg(num_args(0..=1))]
        target_path: Option<String>,
    },
    /// 重命名文件/目录
    #[command(name = "rename", alias = "rn")]
    Rename {
        /// 被重命名的文件/目录路径
        path: String,
        /// 重命名后的路径
        target_path: String,
    },
    /// 写入文件（连续两次回车结束写入）
    #[command(name = "write", alias = "wr")]
    Write {
        /// 写入文件路径
        path: String,
    },
}

fn show_file_info(entry: &DirEntry) {
    let file_name = entry.file_name();
    let file_name = file_name.to_string_lossy().into_owned();

    // 安全获取文件类型
    let file_type = match entry.file_type() {
        Ok(ft) => ft,
        Err(err) => {
            log::warn(&format!("无法获取 [{}] 的类型: {}", file_name, err));
            return;
        }
    };

    // 安全获取文件大小（处理元数据获取失败的情况）
    let size = match entry.metadata() {
        Ok(meta) => format!("{:.2}KB", meta.len() as f64 / 1024.0),
        Err(err) => {
            log::warn(&format!("无法获取 [{}] 的大小: {}", file_name, err));
            "未知大小".to_string()
        }
    };

    // 区分文件/目录显示（目录末尾加 /）
    if file_type.is_file() {
        println!("{: <20} {: <20}", file_name, size);
    } else if file_type.is_dir() {
        println!("{: <20} {: <20}", format!("{}/", file_name), size);
    } else {
        println!("{: <20} {: <20}", file_name, size); // 其他类型（如链接）
    }
}

pub fn handle_command(file_commands: &FileCommands) {
    match file_commands {
        FileCommands::List { target_path } => {
            // 解析路径（处理默认路径和错误）
            let target = match resolve_path(target_path) {
                Ok(path) => path,
                Err(_) => return,
            };

            // 打印表头（无论是否有路径都显示）
            println!("{: <20} {: <20}", "名称", "大小");

            // 安全读取目录（避免 panic）
            let entries = match fs::read_dir(&target) {
                Ok(entries) => entries,
                Err(err) => {
                    log::error(&format!("读取目录失败: {}", err));
                    return;
                }
            };

            // 分离目录和文件，先显示目录再显示文件
            let (dirs, files): (Vec<DirEntry>, Vec<DirEntry>) = entries
                .filter_map(Result::ok) // 过滤掉读取失败的条目
                .partition(|entry| entry.file_type().map_or(false, |ft| ft.is_dir()));

            // 显示目录
            for entry in dirs {
                show_file_info(&entry);
            }
            // 显示文件
            for entry in files {
                show_file_info(&entry);
            }
        }

        FileCommands::Open { target_path } => {
            let target = match resolve_path(target_path) {
                Ok(path) => path,
                Err(_) => return,
            };

            // 调用系统默认程序打开文件/目录
            if let Err(err) = opener::open(&target) {
                log::error(&format!("打开失败: {}", err));
            } else {
                log::success(&format!("已打开: {}", target.display()));
            }
        }

        FileCommands::Delete { target_path } => {
            let target = match resolve_path(target_path) {
                Ok(path) => path,
                Err(_) => return,
            };

            // 区分文件和目录，分别处理删除
            if target.is_file() {
                if let Err(err) = fs::remove_file(&target) {
                    log::error(&format!("删除文件失败: {}", err));
                } else {
                    log::success(&format!("文件已删除: {}", target.display()));
                }
            } else if target.is_dir() {
                if let Err(err) = fs::remove_dir_all(&target) {
                    log::error(&format!("删除目录失败: {}", err));
                } else {
                    log::success(&format!("目录已删除: {}", target.display()));
                }
            } else {
                log::error(&format!("目标不存在或不支持: {}", target.display()));
            }
        }

        FileCommands::Rename { path, target_path } => {
            // 解析源路径和目标路径
            let source = match resolve_path(&Some(path.clone())) {
                Ok(p) => p,
                Err(_) => return,
            };
            let dest = match resolve_path(&Some(target_path.clone())) {
                Ok(p) => p,
                Err(_) => return,
            };

            // 执行重命名
            if let Err(err) = fs::rename(&source, &dest) {
                log::error(&format!("重命名失败: {}", err));
            } else {
                log::success(&format!(
                    "已重命名: {} -> {}",
                    source.display(),
                    dest.display()
                ));
            }
        }

        FileCommands::Write { path } => {
            // 安全创建文件（避免 panic）
            let mut file = match fs::File::create(path) {
                Ok(f) => f,
                Err(err) => {
                    log::error(&format!("创建文件失败: {}", err));
                    return;
                }
            };

            log::info("请输入内容（连续两次回车结束）:");
            let stdin = io::stdin();
            let mut lines_iter = stdin.lock().lines();

            let mut input = String::new();
            let mut consecutive_empty = 0; // 连续空行计数器

            while let Some(Ok(line)) = lines_iter.next() {
                if line.trim().is_empty() {
                    consecutive_empty += 1;
                    // 连续两次空行则结束
                    if consecutive_empty >= 2 {
                        log::info("检测到连续空行，结束写入");
                        break;
                    }
                    // 第一次空行仍写入（保留空行）
                    input.push('\n');
                } else {
                    consecutive_empty = 0; // 非空行重置计数器
                    input.push_str(&line);
                    input.push('\n'); // 补充换行符
                }
            }

            // 写入文件
            if let Err(err) = file.write_all(input.as_bytes()) {
                log::error(&format!("写入文件失败: {}", err));
            } else {
                log::success(&format!("文件已写入: {}", path));
            }
        }
    }
}
