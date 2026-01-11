use crate::blobstorage;
use crate::shared;

use blobstorage::utils::types::CsvRow;
use shared::statics;
use std::collections::HashMap;
use std::fs::File;

pub fn get_or_init_container_cache(
    account_name: &str,
    container_name: &str,
) -> Option<HashMap<String, CsvRow>> {
    let csv_name = format!("{}.csv", container_name);

    let mut path = statics::blobstorage_cache_path();
    path.push(account_name);
    path.push(csv_name);

    if !path.is_file() {
        return Some(HashMap::<String, CsvRow>::new());
    }

    let file = File::open(path).expect("Failed to open cache file.");

    let mut reader = csv::Reader::from_reader(file);

    let rows: HashMap<String, CsvRow> = reader
        .deserialize::<CsvRow>()
        .filter_map(Result::ok)
        .map(|r| (r.name.clone(), r))
        .collect();

    Some(rows)
}
