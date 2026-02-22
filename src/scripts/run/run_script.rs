use crate::{
    scripts::utils,
    shared::{terminal::COLORS},
};
use std::process;

pub fn run_script() {
    let Some((script, path)) = utils::select_script() else {
        return;
    };

    println!(
        "{yellow}Running {white}{}{reset}\n",
        script,
        yellow = COLORS.Yellow,
        white = COLORS.White,
        reset = COLORS.Reset
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
