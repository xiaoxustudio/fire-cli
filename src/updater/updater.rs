use self_update::cargo_crate_version;
use std::error::Error;

pub fn update() -> Result<(), Box<dyn Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("xiaoxustudio")
        .repo_name("fire-cli")
        .bin_name("fire-cli")
        .show_download_progress(true)
        .target("fire-cli.exe")
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    match status {
        self_update::Status::UpToDate(_) => println!("已经是最新版本的!"),
        self_update::Status::Updated(version) => println!("更新至 {}!", version),
    }

    Ok(())
}
