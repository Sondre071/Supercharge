use crate::shared::{
    menu::{self, Cursor, NONE},
    statics,
    terminal::COLORS,
};
use std::{env, fs, path::PathBuf, process};

pub fn main() {
    let path = statics::scripts_dir();

    let directories = get_directories(&path);
    let choices = directories.iter().map(|e| e.as_str()).collect();

    if directories.is_empty() {
        println!("{}No scripts found.{}", COLORS.Yellow, COLORS.Reset);
        process::exit(0)
    }

    let folder = {
        let mut menu = Cursor::new("Select script folder", NONE, choices);
        let Some((folder, _)) = menu::run(&mut menu) else {
            return;
        };

        folder
    };

    let script = {
        let scripts = {
            let mut subdirectory_path = path.clone();
            subdirectory_path.push(&folder);

            get_files(&subdirectory_path)
        };

        let options = scripts.iter().map(|e| e.as_str()).collect();

        let subheader = {
            let current_dir = env::current_dir().expect("Failed to get current terminal location");

            let skip = current_dir.iter().count().saturating_sub(3);

            let displayed_path: PathBuf = current_dir.iter().skip(skip).collect();

            format!("Current directory: {}", displayed_path.display())
        };

        if let Some((script, _)) = menu::run(&mut Cursor::new(
            "Select script",
            Some(vec![subheader.as_str(), ""]),
            options,
        )) {
            script
        } else {
            return;
        }
    };

    let script_path: PathBuf = path.clone().join(folder).join(&script);

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
            script_path.to_str().unwrap(),
        ])
        .spawn()
        .expect("Failed to run script.")
        .wait();

    process::exit(0);
}

fn get_directories(path: &PathBuf) -> Vec<String> {
    let directories = fs::read_dir(path).expect("Failed to open scripts folder.");

    let folder_names: Vec<String> = directories
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let file_type = e.file_type().unwrap();
            let file_name = e.file_name().to_string_lossy().to_string();

            file_type.is_dir() && !file_name.starts_with('.')
        })
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    folder_names
}

fn get_files(path: &PathBuf) -> Vec<String> {
    fs::read_dir(path)
        .expect("Failed to read script directory path.")
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().unwrap().is_file())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect()
}
