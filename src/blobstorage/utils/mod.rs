mod tests;

pub mod types;

mod select_directory;
pub use select_directory::select_directory;

mod select_storage_account;
pub use select_storage_account::select_storage_account;

mod get_blob_settings;
pub use get_blob_settings::get_blob_settings;

pub fn parse_container_name(name: &str) -> String {
    return name.to_lowercase().replace(" ", "-").replace("_", "-");
}
