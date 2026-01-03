use crate::blobstorage;

use blobstorage::types::LocalFile;
use blobstorage::utils::types::StorageAccount;

use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use std::time;
use time::SystemTime;

pub fn upload_blob(
    account: &StorageAccount,
    container_name: &str,
    file: &LocalFile,
    file_content: Vec<u8>,
) {
    let url = format!(
        "{}{}/{}?{}&timeout=600",
        account.blob_endpoint, container_name, file.name, account.shared_access_signature
    );

    let client = reqwest::blocking::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );

    let x_ms_date = httpdate::fmt_http_date(SystemTime::now());

    headers.insert(
        HeaderName::from_static("x-ms-date"),
        HeaderValue::from_str(&x_ms_date).expect("Failed to apply timestamp."),
    );

    headers.insert(
        HeaderName::from_static("x-ms-blob-type"),
        HeaderValue::from_static("blockblob"),
    );

    let response = client
        .put(url)
        .body(file_content)
        .headers(headers)
        .send()
        .expect("Failed to upload file.");

    let status = response.status();

    if !status.is_success() {
        let body_text = response
            .text()
            .unwrap_or_else(|_| String::from("Unable to parse response body."));

        panic!("Non-successful HTTP status: {}, {}", status, body_text)
    }
}
