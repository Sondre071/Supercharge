use crate::blobstorage;
use crate::utils::{menu, terminal};

use blobstorage::api;
use blobstorage::types::FileDiff;
use blobstorage::utils;
use blobstorage::utils::types::StorageAccount;
use create_diff::create_diff;
use print_diff::print_diff;
use select_local_container::select_local_container;
use sync_files::sync_files;
use terminal::COLORS;

mod create_diff;
mod print_diff;
mod select_local_container;
mod sync_files;

pub fn sync_container(account: &StorageAccount, all: bool) {
    let containers = select_local_container(all);
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
                _ => blobstorage::main(),
            }
        }

        terminal::set_cursor_visibility(false);

        let local_files = utils::fetch_local_files(&path);

        let diff: FileDiff = create_diff(local_files, blob_files.unwrap());

        print_diff(&diff);

        if diff.sync_available() {
            let choice = menu::run("Synchronize?", None, vec!["Yes", "No"]).unwrap();

            if choice == "Yes" {
                sync_files(account, &name, diff);
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

    terminal::set_cursor_visibility(true);
}
