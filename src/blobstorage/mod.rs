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
        let data = get_blob_data();

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

    let choice = menu::r#loop::run(
        "Blob Storage",
        None,
        vec!["Browse containers", "Sync container"],
    )
    .unwrap();

    match choice {
        "Browse containers" => browse_containers(&account),
        "Sync container" => sync_container(&account),
        _ => {}
    }
}

fn sync_container(account: &StorageAccount) {
    let Some(name) = select_directory() else {
        return;
    };

    println!(
        "{}Selected container: {}{}",
        COLORS.DarkGray, &name, COLORS.Gray
    );

    let blobs = fetch_blobs(account, name.as_str());

    if blobs.is_none() {
        let choice = menu::r#loop::run(
            "Container not found",
            Some(vec!["Create one?", ""]),
            vec!["Yes", "No"],
        )
        .unwrap();

        match choice {
            "Yes" => create_container(account, &name),
            _ => {}
        }
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

fn select_directory() -> Option<String> {
    let path = rfd::FileDialog::new().pick_folder().unwrap();
    let unparsed_name = path.file_name().unwrap().to_str().unwrap();

    let name = unparsed_name.to_lowercase().replace(" ", "-");

    Some(name)
}
