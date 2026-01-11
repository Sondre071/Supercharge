use crate::blobstorage;
use crate::shared;

use blobstorage::types::LocalFile;
use blobstorage::utils::types::CsvRow;
use shared::statics;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};

pub fn set_container_cache(
    account_name: &str,
    container_name: &str,
    files: &HashMap<String, LocalFile>,
) {
    let rows = files.values().map(|f| CsvRow {
        name: f.name.clone(),
        content_md5: f.content_md5.clone(),
        length: f.content_length,
    });

    let csv_name = format!("{}.csv", container_name);

    let mut path = statics::blobstorage_cache_path();
    path.push(account_name);
    path.push(csv_name);

    if !path.is_file() {
        fs::create_dir_all(path.parent().unwrap()).expect("Failed to create storage account cache.");
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open cache file.");

    let mut writer = csv::Writer::from_writer(file);

    for row in rows {
        writer
            .serialize(row.clone())
            .expect("Failed to write row to CSV.");
    }

    writer.flush().unwrap();
}
