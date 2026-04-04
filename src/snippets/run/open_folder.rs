use crate::shared::statics;

use std::process;

pub fn open_folder() {
    let path = statics::snippets_dir();

    let _ = process::Command::new("nvim")
        .args([path])
        .spawn()
        .expect("Failed to open neovim.")
        .wait();

    process::exit(0);
}
