use crate::api::types::BlobEnumerationResults;
use crate::blobstorage::types::*;
use crate::data::types::StorageAccount;
use crate::terminal::colors;

use colors::COLORS;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use std::time;
use time::SystemTime;

pub fn fetch_containers(account: &StorageAccount) -> Option<Vec<String>> {
    let url = format!(
        "{}?comp=list&{}",
        account.blob_endpoint, account.shared_access_signature
    );

    let client = reqwest::blocking::Client::new();

    let response = client
        .get(&url)
        .send()
        .expect("Failed to fetch containers.");

    if !response.status().is_success() {
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            println!("Storage account not found.");
            return None;
        }

        let status = response.status();
        let body_text = response
            .text()
            .unwrap_or_else(|_| String::from("Unable to read response body"));

        panic!("Non-success HTTP status: {}, {}", status, body_text);
    }

    let body_text = response.text().expect("Failed to read response body");

    let result: ContainerEnumerationResults =
        serde_xml_rs::from_str(&body_text).expect("Failed to parse XML response");

    let container_names: Vec<String> = result
        .containers
        .container
        .iter()
        .map(|c| c.name.clone())
        .collect();

    Some(container_names)
}

pub fn fetch_blobs(account: &StorageAccount, container: &str) -> Option<Vec<BlobFile>> {
    let url = format!(
        "{}{}?restype=container&comp=list&{}",
        account.blob_endpoint, container, account.shared_access_signature
    );

    let client = reqwest::blocking::Client::new();

    let response = client.get(url).send().expect("Failed to fetch blobs.");

    if !response.status().is_success() {
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return None;
        }

        let status = response.status();
        let body_text = response
            .text()
            .unwrap_or_else(|_| String::from("Unable to read response body"));

        panic!("Non-success HTTP status: {}, {}", status, body_text);
    }

    let body_text = response.text().expect("Failed to read response body");

    let result: BlobEnumerationResults =
        serde_xml_rs::from_str(&body_text).expect("Failed to parse XML response");

    let blobs = result
        .blobs
        .blob
        .into_iter()
        .map(|b| BlobFile::from(b))
        .collect();

    Some(blobs)
}

pub fn create_container(account: &StorageAccount, name: &str) {
    let url = format!(
        "{}{}?restype=container&{}",
        account.blob_endpoint, name, account.shared_access_signature
    );

    let client = reqwest::blocking::Client::new();

    let response = client
        .put(url)
        .body("")
        .send()
        .expect(format!("Failed to create container: {}", name).as_str());

    let status = response.status();

    if !status.is_success() {
        let body_text = response
            .text()
            .unwrap_or_else(|_| String::from("Unable to parse response body."));

        panic!("Non-successful HTTP status: {}, {}", status, body_text)
    }

    println!(
        "{}Container: {}{}{} created!{}\n",
        COLORS.Yellow, COLORS.White, name, COLORS.Yellow, COLORS.Gray
    );
}

pub fn upload_file(
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
