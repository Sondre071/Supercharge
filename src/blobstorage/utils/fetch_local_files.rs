use crate::{
    blobstorage::{
        types::LocalFile,
        utils::types::{CsvRow, StorageAccount},
    },
    shared::terminal::codes::*,
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
    account: &StorageAccount,
) -> HashMap<String, LocalFile> {
    let files: HashMap<String, LocalFile> = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let name = e.file_name().to_str().unwrap();
            let kb = e.metadata().unwrap().len() / 1024;

            print!(
                "\r{CLEAR_LINE}{TEXT_COLOR}Hashing: {TEXT_HIGHLIGHTED_COLOR}{}{TEXT_HIGHLIGHTED_FADED_COLOR} ({} kb){RESET_COLOR}",
                name,
                kb,
            );
            io::stdout().flush().unwrap();

            let file = LocalFile::from_entry_cached(&e, name, cache, account.disallowed_file_types.clone());

            (file.content_md5.to_owned(), file)
        })
        .collect();

    print!("\r{CLEAR_LINE}");
    io::stdout().flush().unwrap();

    files
}
