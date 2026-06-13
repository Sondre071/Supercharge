use crate::blobstorage;
use crate::shared::terminal;

use blobstorage::utils::types::StorageAccount;
use terminal::codes::*;

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
        .unwrap_or_else(|_| panic!("Failed to create container: {}", name));

    let status = response.status();

    if !status.is_success() {
        let body_text = response
            .text()
            .unwrap_or_else(|_| String::from("Unable to parse response body."));

        panic!("Non-successful HTTP status: {}, {}", status, body_text)
    }

    println!(
        "{INFO_COLOR}Container: {TEXT_COLOR}{}{INFO_COLOR} created!{RESET_COLOR}\n",
        name,
    );
}
