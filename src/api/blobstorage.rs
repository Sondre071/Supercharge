use crate::api::types::*;
use crate::blobstorage::types::*;
use crate::data::types::StorageAccount;

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

    let response = client.get(&url).send().expect("Failed to fetch blobs.");

    if !response.status().is_success() {
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            println!("Storage container not found.");
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

    let blobs: Vec<BlobFile> = result
        .blobs
        .blob
        .into_iter()
        .map(|b| BlobFile {
            name: b.name,
            content_length: b.properties.content_length,
            last_modified: b.properties.last_modified,
            creation_time: b.properties.creation_time,
            version_id: b.version_id,
            content_md5: b.properties.content_md5,
        })
        .collect();

    Some(blobs)
}
