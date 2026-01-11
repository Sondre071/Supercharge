mod tests;

pub mod types;

mod select_directory;
pub use select_directory::select_directory;

mod select_storage_account;
pub use select_storage_account::select_storage_account;

mod get_blob_settings;
pub use get_blob_settings::get_blob_settings;

mod fetch_local_files;
pub use fetch_local_files::fetch_local_files;

mod set_container_cache;
pub use set_container_cache::set_container_cache;

mod get_or_init_container_cache;
pub use get_or_init_container_cache::get_or_init_container_cache;

pub fn parse_container_name(name: &str) -> String {
    name.to_lowercase()
        .replace(" ", "-")
        .replace("_", "-")
        .replace("--", "-")
}
