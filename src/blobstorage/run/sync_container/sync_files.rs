use crate::blobstorage;
use crate::shared::terminal;

use blobstorage::api;
use blobstorage::types::{FileDiff};
use blobstorage::utils::types::StorageAccount;
use std::io::{self, Write};
use terminal::COLORS;

const UPLOAD_THRESHOLD: usize = 20 * 1024 * 1024; // 20 MB

pub fn sync_files(account: &StorageAccount, container_name: &str, diff: FileDiff) {
    terminal::set_cursor_visibility(false);

    for file in diff.new_files.values() {
        let url = format!(
            "{}{}/{}?{}",
            account.blob_endpoint, container_name, file.name, account.shared_access_signature
        );

        println!(
            "{yellow}Uploading {white}{}{gray} ({} kb){reset}",
            file.name,
            file.content_length / 1024,
            yellow = COLORS.Yellow,
            white = COLORS.White,
            gray = COLORS.Gray,
            reset = COLORS.Reset,
        );

        io::stdout().flush().unwrap();

        if file.content_length < UPLOAD_THRESHOLD {
            api::put_blob(&url, file);
        } else {
            api::put_chunked_blob(&url, file, file.content_length);
        }
    }

    // for (_, (local, remote)) in &diff.changed_files { }

    // for (_, file) in &diff.deleted_files { }

    println!(
        "{yellow}\nContainer {white}{}{yellow} updated!{reset}\n",
        container_name,
        white = COLORS.White,
        yellow = COLORS.Yellow,
        reset = COLORS.Reset
    );
}
