use crate::statics;

use super::types::PromptFile;

pub fn get_prompts() -> Vec<PromptFile> {
    let entries =
        std::fs::read_dir(statics::prompts_dir()).expect("Failed to read from prompts folder.");

    let prompts: Vec<PromptFile> = entries
        .filter_map(|file| {
            let file = file.expect("Failed to parse file.");
            let file_name = file.file_name().to_str().unwrap().to_string();
            let file_path = file.path();

            Some(PromptFile {
                name: file_name,
                path: file_path,
            })
        })
        .collect();

    prompts
}
