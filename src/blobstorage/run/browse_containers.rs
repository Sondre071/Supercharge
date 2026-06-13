use crate::{
    blobstorage::{api, utils::select_storage_account},
    shared::{
        menu::{self, Cursor, NONE},
        terminal::codes::*,
    },
};

pub fn browse_containers() {
    let Some(account) = select_storage_account() else { return };

    let container = match menu::run(&mut Cursor::new(
        "Select container",
        NONE,
        api::fetch_containers(&account).unwrap(),
        None,
    )) {
        Some((container, _)) => container,
        _ => return,
    };

    let blobs = api::fetch_blobs(&account, &container).unwrap();

    for blob in blobs.values() {
        println!(
            "{INFO_COLOR}Name:      {TEXT_COLOR}{}{RESET_COLOR}",
            blob.name
        );

        println!(
            "{INFO_COLOR}Size:      {TEXT_FADED_COLOR}{} kb{RESET_COLOR}",
            blob.content_length / 1024,
        );

        println!(
            "{INFO_COLOR}Modified:  {SUCCESS_COLOR}{}{RESET_COLOR}",
            blob.last_modified
        );

        println!()
    }
}
