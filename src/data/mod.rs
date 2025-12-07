use std::env;
use std::fs::File;
use std::io::BufReader;
use types::{BlobStorageData, OpenRouterData};

pub mod types;

pub fn get_blob_data() -> BlobStorageData {
    let mut path = env::home_dir().unwrap();
    path.push(".supercharge");
    path.push("data");
    path.push("blob-storage.json");

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let data: BlobStorageData = serde_json::from_reader(reader).unwrap();

    data
}

pub fn get_openrouter_data() -> OpenRouterData {
    let mut path = env::home_dir().unwrap();
    path.push(".supercharge");
    path.push("data");
    path.push("supercharge.json");

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let data: OpenRouterData = serde_json::from_reader(reader).unwrap();

    data
}
