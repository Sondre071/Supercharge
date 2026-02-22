use crate::{
    blobstorage::{types::LocalFile, utils::types::CsvRow},
    shared::terminal::{ACTIONS, COLORS},
};
use std::{
    collections::HashMap,
    io::{self, Write},
    path::PathBuf,
};
use walkdir::WalkDir;

pub fn fetch_local_files(
    path: &PathBuf,
    cache: &Option<HashMap<String, CsvRow>>,
) -> HashMap<String, LocalFile> {
    let files: HashMap<String, LocalFile> = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let name = e.file_name().to_str().unwrap();
            let kb = e.metadata().unwrap().len() / 1024;

            print!(
                "\r{clear_line}{yellow}Hashing: {white}{}{gray} ({} kb){reset}",
                name,
                kb,
                clear_line = ACTIONS.ClearLine,
                yellow = COLORS.Yellow,
                white = COLORS.White,
                gray = COLORS.Gray,
                reset = COLORS.Reset
            );
            io::stdout().flush().unwrap();

            let file = LocalFile::from_entry_cached(&e, name, cache);

            (file.content_md5.to_owned(), file)
        })
        .collect();

    print!("\r{clear_line}", clear_line = ACTIONS.ClearLine);
    io::stdout().flush().unwrap();

    files
}
