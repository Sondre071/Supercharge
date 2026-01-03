use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct BlobStorageData {
    pub storage_accounts: Vec<StorageAccount>,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone, Debug)]
pub struct StorageAccount {
    pub name: String,
    pub local_files_path: String,
    pub connection_string: String,

    #[serde(default)]
    pub blob_endpoint: String,

    #[serde(default)]
    pub queue_endpoint: String,

    #[serde(default)]
    pub file_endpoint: String,

    #[serde(default)]
    pub table_endpoint: String,

    #[serde(default)]
    pub shared_access_signature: String,
}
