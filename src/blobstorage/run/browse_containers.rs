use crate::blobstorage;
use crate::terminal;
use crate::menu;

use blobstorage::utils::data::types::StorageAccount;
use blobstorage::api;
use terminal::colors::COLORS;


pub fn browse_containers(account: &StorageAccount) {
    let containers = api::fetch_containers(account).unwrap();

    let mut options: Vec<&str> = containers.iter().map(|s| s.as_str()).collect();
    options.push("Back");

    let container = menu::run("Containers", None, options).unwrap();

    let blobs = api::fetch_blobs(&account, container).unwrap();

    for blob in blobs {
        println!("{}Name: {}", COLORS.White, blob.name);

        println!("{}Size: {}kb", COLORS.Yellow, blob.content_length / 1024);

        println!("{}Last modified: {}", COLORS.Green, blob.last_modified);

        println!("{}", COLORS.Gray)
    }
}
