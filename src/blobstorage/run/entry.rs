use crate::blobstorage;
use crate::menu;

use blobstorage::run;
use blobstorage::utils;

pub fn run() {
    let account = utils::select_storage_account();

    let result = menu::run(
        "Blob Storage",
        None,
        vec!["Browse containers", "Sync container", "Back"],
    )
    .unwrap();

    match result {
        "Browse containers" => run::browse_containers(&account),
        "Sync container" => run::sync_container(&account),
        _ => {}
    }
}
