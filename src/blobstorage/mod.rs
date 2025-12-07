use crate::binary;
use crate::data;
use crate::menu;

pub fn run() {
    get_containers();
}

pub fn get_containers() {
    let data = data::get_blob_data();

    let container_name = menu::r#loop::run(
        "Select storage account",
        None,
        data.storage_accounts
            .iter()
            .map(|i| i.name.as_str())
            .collect::<Vec<&str>>(),
    )
    .unwrap();

    let container = data
        .storage_accounts
        .iter()
        .find(|e| e.name == container_name)
        .unwrap();

    let args = vec![
        "--connectionstring".to_string(),
        container.connection_string.clone(),
    ];

    let mut binary_path = std::env::current_exe().unwrap();
    binary_path.pop();
    binary_path.push("bin");
    binary_path.push("blobstorage");
    binary_path.push("fetch_containers.exe");

    let response = match binary::run_and_collect_lines(binary_path.to_str().unwrap(), args) {
        Ok(r) => r,
        Err(e) => {
            panic!("Failed to run binary: {}", e);
        }
    };

    response
        .iter()
        .enumerate()
        .for_each(|(_, v)| println!("\x1b[0;93m{}\x1b[0m", v));
}
