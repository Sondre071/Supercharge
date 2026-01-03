use crate::blobstorage::utils::data::types::StorageAccount;
use super::types::*;

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
