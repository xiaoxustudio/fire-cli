use clap::Subcommand;

use crate::{log, util::resolve_path};
#[derive(Subcommand, Debug)]
pub enum LinkCommands {
    Create {
        /// 处理路径，默认为当前路径
        #[arg( num_args(0..=1))]
        target_path: Option<String>,
    },
}

pub fn handle_command(link_commands: &LinkCommands) {
    match link_commands {
        LinkCommands::Create { target_path } => {
            let _target = match resolve_path(target_path) {
                Ok(path) => path,
                Err(_) => return,
            };
            // todo
            log::info("todo");
        }
    }
}
