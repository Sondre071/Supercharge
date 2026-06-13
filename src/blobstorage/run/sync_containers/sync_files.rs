use crate::{
    blobstorage::{api, types::FileDiff, utils::types::StorageAccount},
    shared::terminal::{self, codes::*},
};

use std::io::{self, Write};

const UPLOAD_THRESHOLD: usize = 20 * 1024 * 1024; // 20 MB

pub fn sync_files(account: &StorageAccount, container_name: &str, diff: FileDiff) {
    terminal::set_cursor_visibility(false);

    for file in diff.new_files.values() {
        let url = create_blob_url(account, container_name, &file.name);

        println!(
            "{TEXT_COLOR}Uploading {TEXT_HIGHLIGHTED_COLOR}{}{TEXT_HIGHLIGHTED_FADED_COLOR} ({} kb){RESET_COLOR}",
            file.name,
            file.content_length / 1024,
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
            "{TEXT_COLOR}Renaming {TEXT_HIGHLIGHTED_COLOR}{}{TEXT_COLOR} to {TEXT_HIGHLIGHTED_COLOR}{}{RESET_COLOR}",
            &remote.name, &local.name,
        );

        let source_url = create_blob_url(account, container_name, &remote.name);
        let destination_url = create_blob_url(account, container_name, &local.name);

        api::copy_blob(&source_url, destination_url);
        api::delete_blob(&source_url);
    }

    for file in diff.deleted_files.values() {
        println!(
            "{DANGER_COLOR}Deleting {TEXT_HIGHLIGHTED_COLOR}{}{TEXT_HIGHLIGHTED_FADED_COLOR} ({} kb){RESET_COLOR}",
            file.name,
            file.content_length / 1024,
        );

        let url = create_blob_url(account, container_name, &file.name);

        api::delete_blob(&url);
    }

    println!(
        "{SUCCESS_COLOR}\nContainer {TEXT_HIGHLIGHTED_COLOR}{}{SUCCESS_COLOR} updated!{RESET_COLOR}\n",
        container_name,
    );
}

fn create_blob_url(account: &StorageAccount, container_name: &str, file_name: &str) -> String {
    format!(
        "{}{}/{}?{}",
        account.blob_endpoint, container_name, file_name, account.shared_access_signature
    )
}
