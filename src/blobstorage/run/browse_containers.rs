use crate::blobstorage;
use crate::utils::{menu, terminal};

use blobstorage::api;
use blobstorage::utils::types::StorageAccount;
use terminal::COLORS;

pub fn browse_containers(account: &StorageAccount) {
    let containers = api::fetch_containers(account).unwrap();

    let options: Vec<&str> = containers.iter().map(|s| s.as_str()).collect();

    let container = menu::run("Containers", None, options).unwrap();

    let blobs = api::fetch_blobs(account, container).unwrap();

    for blob in blobs.values() {
        println!(
            "{yellow}Name:      {white}{}{reset}",
            blob.name,
            yellow = COLORS.Yellow,
            white = COLORS.White,
            reset = COLORS.Reset
        );

        println!(
            "{yellow}Size:      {gray}{} kb{reset}",
            blob.content_length / 1024,
            yellow = COLORS.Yellow,
            gray = COLORS.Gray,
            reset = COLORS.Reset
        );

        println!(
            "{yellow}Modified:  {green}{}{reset}",
            blob.last_modified,
            yellow = COLORS.Yellow,
            green = COLORS.Green,
            reset = COLORS.Reset
        );

        println!()
    }
}
