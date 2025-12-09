use crate::statics;
use super::types::BlobStorageData;

use std::fs::File;
use std::io::BufReader;

pub fn get_blob_data() -> BlobStorageData {
    let file = File::open(statics::blobstorage_settings_path())
        .expect("Failed to open Blobstorage settings.");

    let reader = BufReader::new(file);
    let data: BlobStorageData =
        serde_json::from_reader(reader).expect("Failed to deserialize blobstorage settings.");

    data
}
