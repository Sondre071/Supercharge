use crate::api::blobstorage::*;
use crate::data;
use crate::menu;
use crate::terminal;

use data::blobstorage::get_blob_data;
use data::types::StorageAccount;
use terminal::colors::COLORS;

pub mod types;

pub fn run() {
    let account = {
        let data = data::blobstorage::get_blob_data();

        if data.storage_accounts.len() == 0 {
            panic!("No storage accounts found.")
        }

        if data.storage_accounts.len() == 1 {
            data.storage_accounts.first().unwrap().to_owned()
        } else {
            let options = data
                .storage_accounts
                .iter()
                .map(|a| a.name.as_str())
                .collect();
            let name = menu::r#loop::run("Select account", None, options).unwrap();

            data.storage_accounts
                .iter()
                .find(|a| a.name == name)
                .unwrap()
                .to_owned()
        }
    };

    let choice = menu::r#loop::run("Blob Storage", None, vec!["Browse containers"]).unwrap();

    match choice {
        "Browse containers" => browse_containers(&account),
        //"Sync container" => sync_container(),
        _ => {}
    }
}

fn browse_containers(account: &StorageAccount) {
    let containers = fetch_containers(account).unwrap();

    let mut options: Vec<&str> = containers.iter().map(|s| s.as_str()).collect();
    options.push("Back");

    let container = menu::r#loop::run("Containers", None, options).unwrap();

    let blobs = fetch_blobs(&account, container).unwrap();

    for blob in blobs {
        println!("{}Name: {}", COLORS.White, blob.name);

        println!("{}Size: {}kb", COLORS.Yellow, blob.content_length / 1024);

        println!("{}Last modified: {}", COLORS.Green, blob.last_modified);

        println!("{}", COLORS.Gray)
    }
}

fn select_scope() -> Option<(StorageAccount, String)> {
    let data = get_blob_data();

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
