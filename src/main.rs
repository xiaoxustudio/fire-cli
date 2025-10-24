// main.rs
mod file;
mod link;
mod log;
mod util;

use clap::{Parser, Subcommand};
use file::FileCommands;
use link::LinkCommands;

#[derive(Subcommand, Debug)]
enum Commands {
    /// 文件相关命令
    #[command(subcommand)]
    File(FileCommands),
    /// 链接相关命令
    #[command(subcommand)]
    Link(LinkCommands),
}

#[derive(Parser, Debug)]
#[command(name = "fire-cli")]
#[command(about = "一个简单的实用工具", long_about = None, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::File(file_cmd) => file::handle_command(file_cmd),
        Commands::Link(link_cmd) => link::handle_command(link_cmd),
    }
}
