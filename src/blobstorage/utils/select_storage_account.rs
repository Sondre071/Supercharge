use crate::{
    blobstorage::utils::{self, types::StorageAccount},
    shared::menu::{self, Cursor, NONE},
};

pub fn select_storage_account() -> Option<StorageAccount> {
    let data = utils::get_blob_settings();

    if data.storage_accounts.is_empty() {
        panic!("No storage accounts found.")
    }

    if data.storage_accounts.len() == 1 {
        data.storage_accounts.first().cloned()
    } else {
        let options = data
            .storage_accounts
            .iter()
            .map(|a| a.name.as_str())
            .collect();

        let (name, _) = menu::run(&mut Cursor::new("Select account", NONE, options, None))?;

        data.storage_accounts
            .iter()
            .find(|a| a.name == name)
            .cloned()
    }
}
