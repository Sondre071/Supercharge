use crate::shared::statics;

use std::fs;

pub fn get_snippet_names() -> Vec<String> {
    let files = fs::read_dir(statics::snippets_path()).unwrap();

    files
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().unwrap().is_file()
                && entry.file_name().to_string_lossy().ends_with(".txt")
        })
        .map(|f| {
            f.file_name()
                .to_string_lossy()
                .to_string()
                .strip_suffix(".txt")
                .unwrap()
                .to_owned()
        })
        .collect()
}
