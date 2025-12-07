use crate::binary;
use crate::data;
use crate::menu;

use data::types::StorageAccount;

pub fn run() {
    let (account, container) = select_scope().unwrap();

    let blobs = get_blobs(account, container.as_str());
}

fn select_scope() -> Option<(StorageAccount, String)> {
    let data = data::get_blob_data();

    let account_name = menu::r#loop::run(
        "Select storage account",
        None,
        data.storage_accounts
            .iter()
            .map(|i| i.name.as_str())
            .collect::<Vec<&str>>(),
    )
    .unwrap();

    let account = data
        .storage_accounts
        .iter()
        .find(|e| e.name == account_name)
        .unwrap()
        .clone();

    let path = rfd::FileDialog::new().pick_folder().unwrap();
    let container = path.file_name().unwrap().to_str().unwrap();

    Some((account, container.to_string()))
}

fn get_blobs(account: StorageAccount, container: &str) -> Option<()> {
    let container_name = container.to_lowercase().replace(" ", "-").to_string();

    let args = vec![
        "--connectionstring".to_string(),
        account.connection_string.clone(),
        "--container".to_string(),
        container_name,
    ];

    let mut binary_path = std::env::current_exe().unwrap();
    binary_path.pop();
    binary_path.push("bin");
    binary_path.push("blobstorage");
    binary_path.push("fetch_blobs.exe");

    let result = match binary::run_and_collect_lines(binary_path.to_str().unwrap(), args) {
        Ok(r) => r,
        Err(e) => {
            panic!("Failed to run binary: {}", e);
        }
    };

    match result {
        binary::ProcessResult::Success(result) => {
            if result.is_empty() {
                return Some(())
            }

            result
                .iter()
                .enumerate()
                .for_each(|(_, v)| println!("\x1b[0;93m{}\x1b[0m", v));
        }
        binary::ProcessResult::NotFound => {
            println!("Storage container not found.");
            return None;
        }
    }

    Some(())
}
