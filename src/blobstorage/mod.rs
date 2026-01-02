use crate::api::blobstorage::*;
use crate::blobstorage;
use crate::data;
use crate::menu;
use crate::terminal;

use blobstorage::types::BlobFile;
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
        vec!["Browse containers", "Sync container", "Back"],
    )
    .unwrap();

    match choice {
        "Browse containers" => browse_containers(&account),
        "Sync container" => sync_container(&account),
        _ => super::main(),
    }
}

fn sync_container(account: &StorageAccount) {
    let Some((name, path)) = select_directory() else {
        return;
    };

    println!(
        "{}Selected container: {}{}",
        COLORS.DarkGray, &name, COLORS.Gray
    );

    let blob_files = fetch_blobs(account, name.as_str());

    if blob_files.is_none() {
        let choice = menu::r#loop::run(
            "Container not found",
            Some(vec!["Create one?", ""]),
            vec!["Yes", "No"],
        )
        .unwrap();

        match choice {
            "Yes" => create_container(account, &name),
            _ => super::blobstorage::run(),
        }
    }

    let local_files: Vec<BlobFile> = std::fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|e| BlobFile::from(e))
        .collect();

    compare_files(local_files, blob_files.unwrap());
}

fn compare_files(local_files: Vec<BlobFile>, blob_files: Vec<BlobFile>) {
    println!("Local files");

    for file in local_files {
        println!("{}", file.name);
        println!("{}", file.content_length);
        println!();
    }

    println!("Blob files");

    for file in blob_files {
        println!("{}", file.name);
        println!("{}", file.content_length);
        println!();
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

fn select_directory() -> Option<(String, std::path::PathBuf)> {
    let path = rfd::FileDialog::new().pick_folder().unwrap();
    let unparsed_name = &path.file_name().unwrap().to_str().unwrap();

    let name = parse_container_name(&unparsed_name);
    Some((name, path))
}

fn parse_container_name(name: &str) -> String {
    return name.to_lowercase().replace(" ", "-").replace("_", "-");
}
