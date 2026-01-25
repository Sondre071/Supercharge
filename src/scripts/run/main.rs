use crate::shared::menu;
use crate::shared::statics;
use crate::shared::terminal::COLORS;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

pub fn main() {
    let path = statics::scripts_dir();

    let directories = get_directories(path);
    let choices = directories.iter().map(|e| e.as_str()).collect();

    if directories.is_empty() {
        println!("{}No scripts found.{}", COLORS.Yellow, COLORS.Reset);
        process::exit(0)
    }

    let choice = menu::run("Select script folder", None, choices);

    if let Some(folder) = choice {
        let scripts = {
            let mut subdirectory_path = path.clone();
            subdirectory_path.push(folder);

            get_files(&subdirectory_path)
        };

        let options = scripts.iter().map(|e| e.as_str()).collect();

        let subheader = {
            let current_dir = env::current_dir().expect("Failed to get current terminal location");

            let skip = current_dir.iter().count().saturating_sub(3);

            let displayed_path: PathBuf = current_dir.iter().skip(skip).collect();

            format!("Current directory: {}", displayed_path.display())
        };

        let choice = menu::run("Select script", Some(vec![subheader.as_str(), ""]), options);

        if let Some(script) = choice {
            let script_path = {
                let mut script_path = path.clone();
                script_path.push(folder);
                script_path.push(script);
                script_path.to_string_lossy().to_string()
            };

            let command_args = format!(
                "Start-Process pwsh -ArgumentList '-ExecutionPolicy Bypass -File \"{}\"'",
                script_path
            );

            let _ = process::Command::new("pwsh")
                .args(["-Command", &command_args])
                .spawn()
                .expect("Failed to run script")
                .wait();
        }
    }
}

fn get_directories(path: &PathBuf) -> Vec<String> {
    let directories = fs::read_dir(path).expect("Failed to open scripts folder.");

    let folder_names: Vec<String> = directories
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let file_type = e.file_type().unwrap();

            file_type.is_dir()
        })
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();

    folder_names
}

fn get_files(path: &PathBuf) -> Vec<String> {
    fs::read_dir(path)
        .expect("Failed to read script directory path.")
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().unwrap().is_dir())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect()
}
