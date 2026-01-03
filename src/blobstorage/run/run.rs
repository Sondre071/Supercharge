use crate::blobstorage;
use crate::menu;

use blobstorage::run;
use blobstorage::utils;

pub fn run() {
    let account = utils::select_storage_account();

    if let Some(result) = menu::run(
        "Blob Storage",
        None,
        vec!["Browse containers", "Sync container", "Back"],
    ) {
        match result {
            "Browse containers" => run::browse_containers(&account),
            "Sync container" => run::sync_container(&account),
            _ => crate::main(),
        }
    } else {
        crate::main();
    }
}
