use crate::blobstorage;
use crate::menu;

use blobstorage::utils;
use blobstorage::utils::types::StorageAccount;

pub fn select_storage_account() -> StorageAccount {
    let data = utils::get_blob_settings();

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
}
