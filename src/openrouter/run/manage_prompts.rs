use crate::shared::statics;

use std::process;

pub fn manage_prompts() {
    let path = statics::prompts_dir();

    let _ = process::Command::new("nvim")
        .arg(path)
        .spawn()
        .expect("Failed to open neovim.")
        .wait();
}