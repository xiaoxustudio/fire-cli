mod file;
mod link;
mod log;
mod open;
mod store;
mod updater;
mod util;

use std::process;

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
    /// 打开相关命令
    #[command(name = "open", alias = "o")]
    Open { target: String },
    /// 将应用程序更新至最新版本
    Update,
}

#[derive(Parser, Debug)]
#[command(name = "fire-cli")]
#[command(about = "一个简单的实用工具", long_about = None, version)]
#[command(author = "xuran")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::File(file_cmd) => file::handle_command(file_cmd),
        Commands::Link(link_cmd) => link::handle_command(link_cmd),
        Commands::Open { target } => open::link::handle_coommand(target),
        Commands::Update => {
            println!("Checking for updates...");
            match updater::updater::update() {
                Ok(_) => {
                    println!("Update successful. Please restart the application.");
                    process::exit(0); // 退出，让用户手动重启
                }
                Err(e) => {
                    eprintln!("Error during update: {}", e);
                    eprintln!("Please try again later or update manually.");
                    process::exit(1);
                }
            }
        }
    }
}
