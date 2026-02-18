use crate::blobstorage;
use crate::shared::menu::{self, Cursor};

use blobstorage::utils;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn select_local_container(all: bool) -> HashMap<String, PathBuf> {
    let mut containers: HashMap<String, PathBuf> = HashMap::new();

    if all {
        let parent_dir = utils::select_directory().unwrap();
        let parent_path = parent_dir.1;

        let (confirm, _) = menu::run(&mut Cursor::new(
            "Correct folder?",
            Some(vec![&parent_dir.0, ""]),
            vec!["Yes", "No"],
        ))
        .unwrap();

        if confirm == "No" {
            return containers;
        }

        for entry in fs::read_dir(&parent_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let unparsed_name = entry.file_name().to_string_lossy().to_string();

            if unparsed_name.starts_with('.') {
                continue;
            }

            let name = utils::parse_container_name(&unparsed_name);

            containers.insert(name, path);
        }
    } else {
        let dir = utils::select_directory().unwrap();
        containers.insert(dir.0, dir.1);
    }

    containers
}
