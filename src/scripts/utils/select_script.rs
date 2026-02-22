use crate::shared::{
    menu::{self, Cursor, NONE},
    statics,
    terminal::COLORS,
};

use std::{fs, path::PathBuf};

pub fn select_script() -> Option<(String, PathBuf)> {
    let mut current_path = statics::scripts_dir();

    let directories = get_directories(&current_path);

    if directories.is_empty() {
        println!("{}No folders found.{}", COLORS.Yellow, COLORS.Reset);
        return None;
    }

    let (folder, _) = menu::run(&mut Cursor::new("Select folder", NONE, directories))?;
    current_path.push(folder);

    let scripts = get_files(&current_path);
    if scripts.is_empty() {
        println!("{}No scripts found.{}", COLORS.Yellow, COLORS.Reset);
        return None;
    }

    let (script, _) = menu::run(&mut Cursor::new("Select script", NONE, scripts))?;
    current_path.push(&script);

    Some((script, current_path))
}

fn get_files(path: &PathBuf) -> Vec<String> {
    fs::read_dir(path)
        .expect("Failed to read script directory path.")
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().unwrap().is_file())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect()
}

fn get_directories(path: &PathBuf) -> Vec<String> {
    let directories = fs::read_dir(path).expect("Failed to open scripts folder.");

    directories
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let file_type = e.file_type().unwrap();
            let file_name = e.file_name().to_string_lossy().to_string();

            file_type.is_dir() && !file_name.starts_with('.')
        })
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect()
}
