use crate::blobstorage;
use crate::utils::terminal;

use blobstorage::types::FileDiff;
use blobstorage::utils::types::StorageAccount;
use blobstorage::api;
use terminal::{COLORS};

pub fn sync_files(
    account: &StorageAccount,
    container_name: &str,
    diff: FileDiff
) {
    terminal::set_cursor_visibility(false);

    for file in diff.new_files.values() {
        api::upload_file(account, container_name, file);
    }
    
    // for (_, (local, remote)) in &diff.changed_files { }
    
    // for (_, file) in &diff.deleted_files { }

    println!(
        "{yellow}\nContainer {white}{}{yellow} updated!{reset}\n",
        container_name,
        white = COLORS.White,
        yellow = COLORS.Yellow,
        reset = COLORS.Reset
    );
}