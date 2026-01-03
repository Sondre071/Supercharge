use std::collections::HashMap;

use crate::statics;

use std::fs::File;
use std::io::BufReader;
use types::*;

pub mod types;
mod tests;

pub fn get_blob_data() -> BlobStorageData {
    let file = File::open(statics::blobstorage_settings_path())
        .expect("Failed to open Blobstorage settings.");

    let reader = BufReader::new(file);
    let mut data: BlobStorageData =
        serde_json::from_reader(reader).expect("Failed to deserialize blobstorage settings.");

    for account in data.storage_accounts.iter_mut() {
        let map = parse_connection_string(&account.connection_string);

        if let Some(v) = map.get("BlobEndpoint") {
            account.blob_endpoint = v.clone();
        }
        if let Some(v) = map.get("QueueEndpoint") {
            account.queue_endpoint = v.clone();
        }
        if let Some(v) = map.get("FileEndpoint") {
            account.file_endpoint = v.clone();
        }
        if let Some(v) = map.get("TableEndpoint") {
            account.table_endpoint = v.clone();
        }
        if let Some(v) = map.get("SharedAccessSignature") {
            account.shared_access_signature = v.clone();
        }
    }

    data
}

pub fn parse_connection_string(s: &str) -> HashMap<String, String> {
    s.split(';')
        .filter_map(|part| {
            let mut kv = part.splitn(2, '=');
            let key = kv.next()?.trim();
            let value = kv.next()?.trim();
            if key.is_empty() {
                return None;
            }
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

pub fn select_directory() -> Option<(String, std::path::PathBuf)> {
    let path = rfd::FileDialog::new().pick_folder().unwrap();
    let unparsed_name = &path.file_name().unwrap().to_str().unwrap();

    let name = parse_container_name(&unparsed_name);
    Some((name, path))
}

fn parse_container_name(name: &str) -> String {
    return name.to_lowercase().replace(" ", "-").replace("_", "-");
}