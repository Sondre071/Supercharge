use crate::blobstorage;
use crate::utils::terminal;

use blobstorage::types::LocalFile;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use terminal::ACTIONS;
use terminal::COLORS;
use walkdir::WalkDir;

pub fn fetch_local_files(path: &PathBuf) -> HashMap<String, LocalFile> {
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

            let file = LocalFile::from(e);

            (file.content_md5.to_owned(), file)
        })
        .collect();

    print!("\r{clear_line}", clear_line = ACTIONS.ClearLine);
    io::stdout().flush().unwrap();

    files
}
