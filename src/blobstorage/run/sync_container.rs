use crate::blobstorage;
use crate::menu;
use crate::terminal;

use blobstorage::api;
use blobstorage::run;
use blobstorage::types::{BlobFile, LocalFile};
use blobstorage::utils;
use blobstorage::utils::types::StorageAccount;
use terminal::console;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use terminal::ACTIONS;
use terminal::COLORS;
use walkdir::WalkDir;

pub fn sync_container(account: &StorageAccount) {
    let mut containers: HashMap<String, PathBuf> = HashMap::new();

    let choice = menu::run("Single or all?", None, vec!["All", "Single"]).unwrap();

    match choice {
        "All" => {
            let parent_dir = utils::select_directory().unwrap();
            let parent_path = parent_dir.1;

            let confirm = menu::run(
                "Correct folder?",
                Some(vec![&parent_dir.0, ""]),
                vec!["Yes", "No"],
            )
            .unwrap();

            if confirm == "No" {
                return;
            }

            for entry in fs::read_dir(&parent_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_dir() {
                    let unparsed_name = entry.file_name().to_string_lossy().to_string();
                    let name = utils::parse_container_name(&unparsed_name);

                    containers.insert(name, path);
                }
            }
        }
        _ => {
            let dir = utils::select_directory().unwrap();
            containers.insert(dir.0, dir.1);
        }
    }

    let containers_len = containers.len();

    for (name, path) in containers {
        println!(
            "\n{yellow}Selected container: {white}{}{reset}",
            &name,
            yellow = COLORS.Yellow,
            white = COLORS.White,
            reset = COLORS.Reset
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

        console::set_cursor_visibility(false);

        let local_files: Vec<LocalFile> = fetch_local_files(&path);

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

    println!(
        "\n{white}{} {yellow}container(s) synced.{reset}\n",
        containers_len,
        white = COLORS.White,
        yellow = COLORS.Yellow,
        reset = COLORS.Reset
    );
}

fn fetch_local_files(path: &PathBuf) -> Vec<LocalFile> {
    let files = WalkDir::new(&path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let name = e.file_name().to_str().unwrap();
            let kb = e.metadata().unwrap().len() / 1024;

            print!(
                "\r{clear_line}{yellow}Hashing: {white}{}{gray} ({} kb){reset}",
                name,
                kb,
                clear_line = ACTIONS.ClearLine,
                yellow = COLORS.Yellow,
                white = COLORS.White,
                gray = COLORS.Gray,
                reset = COLORS.Reset
            );
            io::stdout().flush().unwrap();

            LocalFile::from(e)
        })
        .collect();

    print!("\r{clear_line}", clear_line = ACTIONS.ClearLine);

    files
}

fn sync_blobs(account: &StorageAccount, container_name: &str, pending_uploads: Vec<LocalFile>) {
    console::set_cursor_visibility(false);

    for file in pending_uploads {
        api::upload_file(account, container_name, &file);
    }

    println!(
        "{yellow}\nContainer {white}{}{yellow} updated!{reset}\n",
        container_name,
        white = COLORS.White,
        yellow = COLORS.Yellow,
        reset = COLORS.Reset
    );
}

fn compare_files(local_files: Vec<LocalFile>, blob_files: Vec<BlobFile>) -> Vec<LocalFile> {
    let mut dupes: Vec<(String, String)> = Vec::new();

    let (local, remote) = compile_hashmaps(local_files, blob_files, &mut dupes);

    if dupes.len() > 0 {
        println!(
            "{yellow}Duplicate files found:{reset}\n",
            yellow = COLORS.Yellow,
            reset = COLORS.Reset
        );

        for (name1, name2) in dupes {
            println!(
                "{white}{}{reset}",
                name1,
                white = COLORS.White,
                reset = COLORS.Reset
            );

            println!(
                "{white}{}{reset}",
                name2,
                white = COLORS.White,
                reset = COLORS.Reset
            );
        }

        panic!("Duplicates. Deal with it.")
    }

    println!(
        "{yellow}Local files:  {white}{}{reset}",
        local.len(),
        yellow = COLORS.Yellow,
        white = COLORS.White,
        reset = COLORS.Reset
    );

    println!(
        "{yellow}Remote files: {white}{}{reset}",
        remote.len(),
        yellow = COLORS.Yellow,
        white = COLORS.White,
        reset = COLORS.Reset
    );

    let mut pending_uploads: Vec<LocalFile> = Vec::new();

    for (hash, file) in &local {
        if !remote.contains_key(hash) {
            pending_uploads.push(file.clone());
        }
    }

    if pending_uploads.len() > 0 {
        println!(
            "{yellow}Pending files: {white}{}{reset}\n",
            pending_uploads.len(),
            yellow = COLORS.Yellow,
            white = COLORS.White,
            reset = COLORS.Reset
        );

        for file in &pending_uploads {
            println!(
                "{yellow}Name:      {white}{}{reset}",
                file.name,
                yellow = COLORS.Yellow,
                white = COLORS.White,
                reset = COLORS.Reset
            );

            println!(
                "{yellow}Size:      {gray}{} kb{reset}",
                file.content_length / 1024,
                yellow = COLORS.Yellow,
                gray = COLORS.Gray,
                reset = COLORS.Reset
            );

            println!(
                "{yellow}Modified:  {green}{}{reset}\n",
                file.last_modified,
                yellow = COLORS.Yellow,
                green = COLORS.Green,
                reset = COLORS.Reset
            );
        }
    } else {
        println!(
            "{green}Container synced.{reset}",
            green = COLORS.Green,
            reset = COLORS.Reset
        );
    }

    pending_uploads
}

fn compile_hashmaps(
    local_files: Vec<LocalFile>,
    blob_files: Vec<BlobFile>,
    dupes: &mut Vec<(String, String)>,
) -> (HashMap<String, LocalFile>, HashMap<String, BlobFile>) {
    let mut local = HashMap::new();
    let mut remote = HashMap::new();

    for f in local_files {
        let hash = f.content_md5.clone();
        let name = f.name.clone();

        if let Some(existing) = local.insert(hash.clone(), f) {
            dupes.push((existing.name, name));
        }
    }

    for f in blob_files {
        let hash = f.content_md5.clone();
        let name = f.name.clone();

        if let Some(existing) = remote.insert(hash.clone(), f) {
            dupes.push((existing.name, name));
        }
    }

    (local, remote)
}
