mod file;
mod log;
use file::FileCommands;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "fire-cli")]
#[command(about = "一个简单的实用工具", long_about = None,version)]
struct Cli {
    #[command(subcommand)]
    file: FileCommands,
}

fn main() {
    let cli = Cli::parse();
    file::handle_command(&cli.file);
}
