use crate::blobstorage::{self, api};
use crate::shared::terminal;

use blobstorage::types::FileDiff;
use blobstorage::utils::types::StorageAccount;
use std::io::{self, Write};
use terminal::COLORS;

const UPLOAD_THRESHOLD: usize = 20 * 1024 * 1024; // 20 MB

pub fn sync_files(account: &StorageAccount, container_name: &str, diff: FileDiff) {
    terminal::set_cursor_visibility(false);

    for file in diff.new_files.values() {
        let url = create_blob_url(account, container_name, &file.name);

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

    for (local, remote) in diff.changed_files.values() {
        println!(
            "{yellow}Renaming {white}{}{yellow} to {white}{}{reset}",
            &remote.name,
            &local.name,
            yellow = COLORS.Yellow,
            white = COLORS.White,
            reset = COLORS.Reset,
        );

        let source_url = create_blob_url(account, container_name, &remote.name);
        let destination_url = create_blob_url(account, container_name, &local.name);

        api::copy_blob(&source_url, destination_url);
        api::delete_blob(&source_url);
    }

    for file in diff.deleted_files.values() {
        println!(
            "{yellow}Deleting {white}{}{gray} ({} kb){reset}",
            file.name,
            file.content_length / 1024,
            yellow = COLORS.Yellow,
            white = COLORS.White,
            gray = COLORS.Gray,
            reset = COLORS.Reset,
        );
        
        let url = create_blob_url(account, container_name, &file.name);
        
        api::delete_blob(&url);
    }

    println!(
        "{yellow}\nContainer {white}{}{yellow} updated!{reset}\n",
        container_name,
        white = COLORS.White,
        yellow = COLORS.Yellow,
        reset = COLORS.Reset
    );
}

fn create_blob_url(account: &StorageAccount, container_name: &str, file_name: &str) -> String {
    format!(
        "{}{}/{}?{}",
        account.blob_endpoint, container_name, file_name, account.shared_access_signature
    )
}
