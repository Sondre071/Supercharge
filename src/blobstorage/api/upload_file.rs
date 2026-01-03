use crate::blobstorage;
use crate::terminal;

use blobstorage::types::LocalFile;
use blobstorage::utils::types::StorageAccount;
use terminal::COLORS;

use std::io::{self, Write};

mod put_blob;
mod put_chunked_blob;

use put_blob::put_blob;
use put_chunked_blob::put_chunked_blob;

const UPLOAD_THRESHOLD: usize = 20 * 1024 * 1024; // 20 MB

pub fn upload_file(account: &StorageAccount, container_name: &str, file: &LocalFile) {
    let url = format!(
        "{}{}/{}?{}",
        account.blob_endpoint, container_name, file.name, account.shared_access_signature
    );

    print!(
        "\r\x1b[2K{}Uploading {}{}{} ({} kb)\n",
        COLORS.Yellow,
        COLORS.White,
        file.name,
        COLORS.Gray,
        file.content_length / 1024,
    );

    io::stdout().flush().unwrap();

    if file.content_length < UPLOAD_THRESHOLD {
        put_blob(&url, file);
    } else {
        put_chunked_blob(&url, file, file.content_length);
    }
}
