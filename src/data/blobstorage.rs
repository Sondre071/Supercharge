use std::collections::HashMap;

use super::types::BlobStorageData;
use crate::statics;

use std::fs::File;
use std::io::BufReader;

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

fn parse_connection_string(s: &str) -> HashMap<String, String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::types::StorageAccount;

    #[test]
    fn test_populate_account_from_connection_string() {
        let mut account = StorageAccount {
            name: "test_account".to_string(),
            local_files_path: "C:/tmp".to_string(),
            connection_string: "BlobEndpoint=https://myblob.blob.core.windows.net/;\
                                QueueEndpoint=https://myblob.queue.core.windows.net/;\
                                FileEndpoint=https://myblob.file.core.windows.net/;\
                                TableEndpoint=https://myblob.table.core.windows.net/;\
                                SharedAccessSignature=sv=2020-08-01&ss=bsop&srt=c&sp=rcsllcaiutaz&se=2021-02-01T02:33:50Z&st=2022-11-17T18:55:10Z&spr=https&sig=wp8yOrAzhTlpaQ1ysL9C9tK3WQ2QyDqEMmg6K1qSX%9Uq%7I"
                .to_string(),

            blob_endpoint: String::new(),
            queue_endpoint: String::new(),
            file_endpoint: String::new(),
            table_endpoint: String::new(),
            shared_access_signature: String::new(),
        };

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

        assert_eq!(
            account.blob_endpoint,
            "https://myblob.blob.core.windows.net/"
        );
        assert_eq!(
            account.queue_endpoint,
            "https://myblob.queue.core.windows.net/"
        );
        assert_eq!(
            account.file_endpoint,
            "https://myblob.file.core.windows.net/"
        );
        assert_eq!(
            account.table_endpoint,
            "https://myblob.table.core.windows.net/"
        );
        assert_eq!(
            account.shared_access_signature,
            "sv=2020-08-01&ss=bsop&srt=c&sp=rcsllcaiutaz&se=2021-02-01T02:33:50Z&st=2022-11-17T18:55:10Z&spr=https&sig=wp8yOrAzhTlpaQ1ysL9C9tK3WQ2QyDqEMmg6K1qSX%9Uq%7I"
        );
    }
}
