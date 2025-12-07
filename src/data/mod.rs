use crate::statics;
use std::fs::File;
use std::io::BufReader;
use types::{BlobStorageData, OpenRouterData};

pub mod types;

pub fn get_blob_data() -> BlobStorageData {
    let file = File::open(statics::blobstorage_settings_path())
        .expect("Failed to open Blobstorage settings.");

    let reader = BufReader::new(file);
    let data: BlobStorageData =
        serde_json::from_reader(reader).expect("Failed to deserialize blobstorage settings.");

    data
}

pub fn get_openrouter_data() -> OpenRouterData {
    let file = File::open(statics::openrouter_settings_path())
        .expect("Failed to open OpenRouter settings.");

    let reader = BufReader::new(file);
    let data: OpenRouterData =
        serde_json::from_reader(reader).expect("Failed to deserialize openrouter settings.");

    data
}
