use crate::blobstorage;

use blobstorage::api::types::BlobEnumerationResults;
use blobstorage::types::BlobFile;
use blobstorage::utils::types::StorageAccount;

use std::collections::HashMap;

pub fn fetch_blobs(account: &StorageAccount, container: &str) -> Option<HashMap<String, BlobFile>> {
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

    let blobs: HashMap<String, BlobFile> = result
        .blobs
        .blob
        .into_iter()
        .map(|f| {
            let file = BlobFile::from(f);
            (file.content_md5.to_owned(), file)
        })
        .collect();

    Some(blobs)
}
