use crate::blobstorage;
use crate::menu;
use crate::terminal;

use blobstorage::api;
use blobstorage::run;
use blobstorage::types::{BlobFile, LocalFile};
use blobstorage::utils;
use blobstorage::utils::types::StorageAccount;

use std::collections::HashMap;
use std::io::{self, Write};
use terminal::COLORS;
use terminal::console;
use walkdir::WalkDir;

pub fn sync_container(account: &StorageAccount) {
    let Some((name, path)) = utils::select_directory() else {
        return;
    };

    println!(
        "{}Selected container: {}{}{}",
        COLORS.Yellow, COLORS.White, &name, COLORS.Gray
    );

    let mut blob_files = api::fetch_blobs(account, name.as_str());

    if blob_files.is_none() {
        let choice = menu::run(
            "Container not found",
            Some(vec!["Create one?", ""]),
            vec!["Yes", "No"],
        )
        .unwrap();

        match choice {
            "Yes" => {
                api::create_container(account, &name);
                blob_files = api::fetch_blobs(account, name.as_str());
            }
            _ => run::run(),
        }
    }

    let local_files: Vec<LocalFile> = WalkDir::new(&path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let name = e.file_name().to_str().unwrap();
            let kb = e.metadata().unwrap().len() / 1024;

            print!(
                "\r\x1b[2K{}Hashing: {}{}{} ({} kb)",
                COLORS.Yellow, COLORS.White, name, COLORS.Gray, kb
            );
            io::stdout().flush().unwrap();

            LocalFile::from(e)
        })
        .collect();

    print!("\r\x1b[2K");
    io::stdout().flush().unwrap();

    let pending_uploads: Vec<LocalFile> = compare_files(local_files, blob_files.unwrap());

    if pending_uploads.len() > 0 {
        let subheader = format!("Pending changes: {}", pending_uploads.len());

        let choice = menu::run(
            "Synchronize?",
            Some(vec![subheader.as_str(), ""]),
            vec!["Yes", "No"],
        )
        .unwrap();

        match choice {
            "Yes" => sync_blobs(account, &name, pending_uploads),
            _ => {}
        }
    }
}

fn sync_blobs(account: &StorageAccount, container_name: &str, pending_uploads: Vec<LocalFile>) {
    for file in pending_uploads {
        api::upload_file(account, container_name, &file);
    }
}

fn compare_files(local_files: Vec<LocalFile>, blob_files: Vec<BlobFile>) -> Vec<LocalFile> {
    let mut duplicates: Vec<(String, String)> = Vec::new();

    let local = {
        let mut map = HashMap::new();

        for f in local_files {
            let hash = f.content_md5.clone();

            if let Some(existing) = map.insert(hash.clone(), f) {
                duplicates.push((existing.name, map[&hash].name.clone()));
            }
        }

        map
    };

    let remote = {
        let mut map = HashMap::new();

        for f in blob_files {
            let hash = f.content_md5.clone();

            if let Some(existing) = map.insert(hash.clone(), f) {
                duplicates.push((existing.name, map[&hash].name.clone()));
            }
        }

        map
    };

    if duplicates.len() > 0 {
        println!("{}Duplicate files found:{}\n", COLORS.Yellow, COLORS.Gray);

        for (name1, name2) in duplicates {
            println!("{}{} and {}{}", COLORS.Yellow, name1, name2, COLORS.Gray);
        }

        panic!("Duplicates. Deal with it.")
    }

    println!(
        "{}Local files: {}{}{}",
        COLORS.Yellow,
        COLORS.White,
        local.len(),
        COLORS.Gray
    );

    println!(
        "{}Remote files: {}{}{}\n",
        COLORS.Yellow,
        COLORS.White,
        remote.len(),
        COLORS.Gray
    );

    let mut pending_uploads: Vec<LocalFile> = Vec::new();

    for (hash, file) in &local {
        if !remote.contains_key(hash) {
            pending_uploads.push(file.clone());
        }
    }

    if pending_uploads.len() > 0 {
        println!(
            "{}Pending files: {}{}{}\n",
            COLORS.Yellow,
            COLORS.White,
            pending_uploads.len(),
            COLORS.Gray
        );
    } else {
        println!("{}Local files are synced.{}", COLORS.Green, COLORS.Gray);
    }

    for file in &pending_uploads {
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
    }

    pending_uploads
}
