use crate::blobstorage::utils::{self, types::StorageAccount};
use crate::shared::menu::{self, Menu};

pub fn select_storage_account() -> StorageAccount {
    let data = utils::get_blob_settings();

    if data.storage_accounts.is_empty() {
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

        let (name, _) = menu::run(&mut Menu::new("Select account", vec![""], options)).unwrap();

        data.storage_accounts
            .iter()
            .find(|a| a.name == name)
            .unwrap()
            .to_owned()
    }
}
