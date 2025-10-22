use colored::*;

/// 打印成功信息（绿色）
pub fn success(message: &str) {
    println!("{} {}", "[SUCCESS]".green().bold(), message);
}

/// 打印错误信息（红色）
pub fn error(message: &str) {
    eprintln!("{} {}", "[ERROR]".red().bold(), message);
}

#[allow(dead_code)]
/// 打印警告信息（黄色）
pub fn warn(message: &str) {
    eprintln!("{} {}", "[WARN]".yellow().bold(), message);
}

#[allow(dead_code)]
/// 打印普通信息（蓝色）
pub fn info(message: &str) {
    println!("{} {}", "[INFO]".blue().bold(), message);
}
