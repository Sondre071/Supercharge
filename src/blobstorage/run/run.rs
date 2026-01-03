use crate::blobstorage;
use crate::menu;

use blobstorage::utils::data;
use blobstorage::run;


pub fn run() {
    let account = {
        let data = data::get_blob_data();

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

            let name = menu::run("Select account", None, options).unwrap();

            data.storage_accounts
                .iter()
                .find(|a| a.name == name)
                .unwrap()
                .to_owned()
        }
    };

    let choice = menu::run(
        "Blob Storage",
        None,
        vec!["Browse containers", "Sync container", "Back"],
    )
    .unwrap();

    match choice {
        "Browse containers" => run::browse_containers(&account),
        "Sync container" => run::sync_container(&account),
        _ => crate::main(),
    }
}
