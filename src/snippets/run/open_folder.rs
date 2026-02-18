use crate::shared::statics;
use std::process;

pub fn open_folder() {
    let _ = process::Command::new("nvim")
        .args([&statics::snippets_path()])
        .spawn()
        .expect("Failed to open neovim.")
        .wait();
}
