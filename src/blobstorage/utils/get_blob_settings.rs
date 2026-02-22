use crate::{blobstorage::utils::types::BlobStorageData, shared::statics};
use std::{collections::HashMap, fs, io};

pub fn get_blob_settings() -> BlobStorageData {
    let file = fs::File::open(statics::blobstorage_settings_path())
        .expect("Failed to open Blobstorage settings.");

    let reader = io::BufReader::new(file);
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
