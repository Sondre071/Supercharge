use crate::shared::statics;
use std::fs;

pub fn get_snippet_names() -> Vec<String> {
    let files = fs::read_dir(statics::snippets_dir()).unwrap();

    files
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().unwrap().is_file()
                && entry.file_name().to_string_lossy().ends_with(".md")
        })
        .map(|f| {
            let name = f.file_name().to_string_lossy().to_string();
            name.trim_end_matches(".md").to_string()
        })
        .collect()
}
