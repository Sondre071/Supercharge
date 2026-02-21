use crate::blobstorage::{api, types::FileDiff, utils};
use crate::shared::{
    menu::{self, Cursor, NONE},
    terminal,
};
use std::process::exit;

use create_diff::create_diff;
use print_diff::print_diff;
use select_local_container::select_local_container;
use sync_files::sync_files;
use terminal::COLORS;

mod create_diff;
mod print_diff;
mod select_local_container;
mod sync_files;

pub fn sync_containers() {
    let account = utils::select_storage_account();

    let all = match menu::run(&mut Cursor::new("Sync all?", NONE, vec!["Yes", "No"])) {
        Some((choice, _)) => choice == "Yes",
        _ => return,
    };

    let Some(containers) = select_local_container(all) else {
        return;
    };

    let containers_len = containers.len();

    for (name, path) in containers {
        println!(
            "\n{yellow}Selected container: {white}{}{reset}",
            &name,
            yellow = COLORS.Yellow,
            white = COLORS.White,
            reset = COLORS.Reset
        );

        let blob_files = {
            let response = api::fetch_blobs(&account, name.as_str());

            response.unwrap_or_else(|| {
                let (choice, _) = menu::run(&mut Cursor::new(
                    "Container not found",
                    Some(vec!["Create one?", ""]),
                    vec!["Yes", "No"],
                ))
                .unwrap();

                match choice.as_str() {
                    "Yes" => {
                        api::create_container(&account, &name);
                        api::fetch_blobs(&account, name.as_str()).unwrap()
                    }
                    _ => exit(0),
                }
            })
        };

        terminal::set_cursor_visibility(false);

        let cache = utils::get_or_init_container_cache(&account.name, &name);
        let local_files = utils::fetch_local_files(&path, &cache);

        utils::set_container_cache(&account.name, &name, &local_files);

        let diff: FileDiff = create_diff(local_files, blob_files);

        print_diff(&diff);

        if diff.sync_available() {
            let (choice, _) =
                menu::run(&mut Cursor::new("Synchronize?", NONE, vec!["Yes", "No"])).unwrap();

            if choice == "Yes" {
                sync_files(&account, &name, diff);
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
