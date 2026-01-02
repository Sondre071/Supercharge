use crate::api::blobstorage::*;
use crate::blobstorage;
use crate::data;
use crate::menu;
use crate::terminal;

use blobstorage::types::BlobFile;
use data::blobstorage::get_blob_data;
use data::types::StorageAccount;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
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
        "{}Selected container: {}{}{}",
        COLORS.Yellow, COLORS.White, &name, COLORS.Gray
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

    let local_files: Vec<BlobFile> = std::fs::read_dir(&path)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|e| BlobFile::from(e))
        .collect();

    let pending_uploads: Vec<BlobFile> = compare_files(local_files, blob_files.unwrap());

    if pending_uploads.len() > 0 {
        let subheader = format!("Pending changes: {}", pending_uploads.len());

        let choice = menu::r#loop::run(
            "Synchronize?",
            Some(vec![subheader.as_str(), ""]),
            vec!["Yes", "No"],
        )
        .unwrap();

        match choice {
            "Yes" => sync_blobs(account, &name, &path, pending_uploads),
            _ => {}
        }
    }
}

fn sync_blobs(
    account: &StorageAccount,
    container_name: &str,
    path: &PathBuf,
    pending_uploads: Vec<BlobFile>,
) {
    for file in pending_uploads {
        let mut file_path = path.to_owned();
        file_path.push(&file.name);

        let file_content = fs::read(file_path).expect("Failed to parse file content.");

        upload_file(account, container_name, &file, file_content);

        println!(
            "{}Uploaded {}{}{}!{}",
            COLORS.Yellow, COLORS.White, file.name, COLORS.Yellow, COLORS.Gray
        );
    }
}

fn compare_files(local_files: Vec<BlobFile>, blob_files: Vec<BlobFile>) -> Vec<BlobFile> {
    let local = {
        let mut map = HashMap::new();

        for f in local_files {
            let hash = f.content_md5.clone();

            if let Some(existing) = map.insert(hash.clone(), f) {
                panic!(
                    "Duplicate hash in {} and {}.",
                    existing.name, map[&hash].name
                )
            }
        }

        map
    };

    let remote = {
        let mut map = HashMap::new();

        for f in blob_files {
            let hash = f.content_md5.clone();

            if let Some(existing) = map.insert(hash.clone(), f) {
                panic!(
                    "Duplicate hash in {} and {}.",
                    existing.name, map[&hash].name
                )
            }
        }

        map
    };

    println!(
        "{}Local files: {}{}",
        COLORS.Yellow,
        local.len(),
        COLORS.Gray
    );
    println!(
        "{}Remote files: {}{}\n",
        COLORS.Yellow,
        remote.len(),
        COLORS.Gray
    );

    println!("{}Unsynced files:{}\n", COLORS.Yellow, COLORS.Gray);

    let mut pending_uploads: Vec<BlobFile> = Vec::new();

    for (hash, file) in &local {
        if !remote.contains_key(hash) {
            println!("{}Name:      {}{}", COLORS.Yellow, COLORS.White, file.name);

            println!(
                "{}Size:      {}{}kb",
                COLORS.Yellow,
                COLORS.White,
                file.content_length / 1024
            );

            println!(
                "{}Modified:  {}{}",
                COLORS.Yellow, COLORS.White, file.last_modified
            );

            println!("{}", COLORS.Gray);

            pending_uploads.push(file.clone());
        }
    }

    pending_uploads
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
