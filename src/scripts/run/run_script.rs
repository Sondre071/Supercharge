use crate::{
    scripts::utils,
    shared::{terminal::codes::*},
};
use std::process;

pub fn run_script() {
    let Some((script, path)) = utils::select_script() else {
        return;
    };

    println!(
        "{INFO_COLOR}Running {TEXT_COLOR}{}{RESET_COLOR}\n",
        script,
    );

    let _ = process::Command::new("pwsh")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "ByPass",
            "-File",
            path.to_str().unwrap(),
        ])
        .spawn()
        .expect("Failed to run script.")
        .wait();
}
