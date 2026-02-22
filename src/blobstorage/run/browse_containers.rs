use crate::{
    blobstorage::{api, utils::select_storage_account},
    shared::{
        menu::{self, Cursor, NONE},
        terminal::COLORS,
    },
};

pub fn browse_containers() {
    let account = select_storage_account();

    let container = match menu::run(&mut Cursor::new(
        "Select container",
        NONE,
        api::fetch_containers(&account).unwrap(),
    )) {
        Some((container, _)) => container,
        _ => return,
    };

    let blobs = api::fetch_blobs(&account, &container).unwrap();

    for blob in blobs.values() {
        println!(
            "{yellow}Name:      {white}{}{reset}",
            blob.name,
            yellow = COLORS.Yellow,
            white = COLORS.White,
            reset = COLORS.Reset
        );

        println!(
            "{yellow}Size:      {gray}{} kb{reset}",
            blob.content_length / 1024,
            yellow = COLORS.Yellow,
            gray = COLORS.Gray,
            reset = COLORS.Reset
        );

        println!(
            "{yellow}Modified:  {green}{}{reset}",
            blob.last_modified,
            yellow = COLORS.Yellow,
            green = COLORS.Green,
            reset = COLORS.Reset
        );

        println!()
    }
}
