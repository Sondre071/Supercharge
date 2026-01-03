use crate::blobstorage;

use blobstorage::types::LocalFile;

use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use std::fs;
use std::time::SystemTime;

pub fn put_blob(url: &str, file: &LocalFile) {
    let file_content = fs::read(file.path.clone()).expect("Failed to parse file content.");

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        HeaderName::from_static("x-ms-date"),
        HeaderValue::from_str(&httpdate::fmt_http_date(SystemTime::now()))
            .expect("Failed to apply timestamp."),
    );
    headers.insert(
        HeaderName::from_static("x-ms-blob-type"),
        HeaderValue::from_static("blockblob"),
    );

    let client = reqwest::blocking::Client::new();

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
