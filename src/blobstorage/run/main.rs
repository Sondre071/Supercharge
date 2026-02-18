use crate::blobstorage;
use crate::shared::menu::{self, Menu};

use blobstorage::run;
use blobstorage::utils;

pub fn main() {
    let account = utils::select_storage_account();

    let (result, _) = menu::run(Menu::new(
        "Blob Storage",
        vec![""],
        vec![
            "Browse containers",
            "Sync container (single)",
            "Sync containers (all)",
            "Back",
        ],
    ))
    .unwrap();

    match result.as_str() {
        "Browse containers" => run::browse_containers(&account),
        "Sync container (single)" => run::sync_container(&account, false),
        "Sync containers (all)" => run::sync_container(&account, true),
        _ => {}
    }
}
