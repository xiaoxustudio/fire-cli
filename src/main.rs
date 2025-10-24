// main.rs
mod file;
mod link;
mod log;
mod util;

use clap::{Parser, Subcommand}; // 导入必要 trait
use file::FileCommands;
use link::LinkCommands;

/// 一级子命令枚举（包含所有顶层子命令）
#[derive(Subcommand, Debug)] // 派生 Subcommand
enum Commands {
    /// 文件相关命令
    #[command(subcommand)]
    File(FileCommands),
    /// 链接相关命令
    #[command(subcommand)]
    Link(LinkCommands),
}

/// 根命令
#[derive(Parser, Debug)]
#[command(name = "fire-cli")]
#[command(about = "一个简单的实用工具", long_about = None, version)]
struct Cli {
    #[command(subcommand)] // 标记为子命令字段
    command: Commands, // 类型为一级子命令枚举
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::File(file_cmd) => file::handle_command(file_cmd),
        Commands::Link(link_cmd) => link::handle_command(link_cmd),
    }
}
