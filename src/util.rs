use std::path::{Path, PathBuf};

use crate::log;

pub fn resolve_path(path_opt: &Option<String>) -> Result<PathBuf, String> {
    let path_str = path_opt.as_deref().unwrap_or(".");
    let path = Path::new(path_str);

    path.canonicalize().map_err(|_| {
        let msg = format!("Invalid path: {}", path_str);
        log::error(&msg);
        msg
    })
}
