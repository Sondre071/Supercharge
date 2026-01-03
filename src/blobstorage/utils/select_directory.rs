use crate::blobstorage;

use blobstorage::utils;

pub fn select_directory() -> Option<(String, std::path::PathBuf)> {
    let path = rfd::FileDialog::new().pick_folder().unwrap();
    let unparsed_name = &path.file_name().unwrap().to_str().unwrap();

    let name = utils::parse_container_name(&unparsed_name);
    Some((name, path))
}
