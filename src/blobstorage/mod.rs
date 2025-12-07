use crate::data;
use crate::menu;
use types::*;

use data::types::StorageAccount;

mod types;

pub fn run() {
    let (account, container) = select_scope().unwrap();

    if let Some(blobs) = get_blobs(account, container.as_str()) {
        if blobs.is_empty() {
            println!("No blobs found in container.");
            return;
        }

        for blob in blobs {
            println!("\x1b[0;93m{}\x1b[0m", blob);
        }
    }
}

fn select_scope() -> Option<(StorageAccount, String)> {
    let data = data::get_blob_data();

    let account_name = menu::r#loop::run(
        "Select storage account",
        None,
        data.storage_accounts
            .iter()
            .map(|i| i.name.as_str())
            .collect::<Vec<&str>>(),
    )
    .unwrap();

    let account = data
        .storage_accounts
        .iter()
        .find(|e| e.name == account_name)
        .unwrap()
        .clone();

    let path = rfd::FileDialog::new().pick_folder().unwrap();
    let container = path.file_name().unwrap().to_str().unwrap();

    Some((account, container.to_string()))
}

fn get_blobs(account: StorageAccount, container: &str) -> Option<Vec<String>> {
    let url = {
        let f_container = container.to_lowercase().replace(" ", "-").to_string();

        let con_values: Vec<&str> = account.connection_string.split(';').collect();

        let base_url = con_values[0].strip_prefix("BlobEndpoint=").unwrap();
        let sv = con_values[4]
            .strip_prefix("SharedAccessSignature=")
            .unwrap();

        format!(
            "{}{}?restype=container&comp=list&{}",
            base_url, f_container, sv
        )
    };

    let client = reqwest::blocking::Client::new();

    println!("{:?}", &url);

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

    println!("{:?}", &body_text);

    let result: BlobEnumerationResults =
        serde_xml_rs::from_str(&body_text).expect("Failed to parse XML response");

    let blob_names: Vec<String> = result.blobs.blob.iter().map(|b| b.name.clone()).collect();

    Some(blob_names)
}
