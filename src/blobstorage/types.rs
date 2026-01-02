use crate::api::types::{Blob};

use serde::Deserialize;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::time;
use base64::{engine::general_purpose, Engine as _};

#[derive(Debug)]
pub struct BlobFile {
    pub name: String,
    pub content_length: usize,
    pub last_modified: String,
    pub creation_time: String,
    pub content_md5: String,
}

impl From<DirEntry> for BlobFile {
    fn from(entry: DirEntry) -> Self {
        let path: PathBuf = entry.path();
        let metadata = entry.metadata().expect("Failed to parse metadata.");

        let name = entry.file_name().to_string_lossy().into_owned();

        let content_length = metadata.len() as usize;

        let last_modified =
            time::SystemTime::from(metadata.modified().expect("Failed to parse last_modified."));
        let last_modified = format!("{:?}", last_modified);

        let creation_time =
            time::SystemTime::from(metadata.created().expect("Failed to get created_time."));
        let creation_time = format!("{:?}", creation_time);

        let bytes = fs::read(path).expect("Failed to parse file content.");
        
        let content_md5 = general_purpose::STANDARD.encode(&bytes);

        BlobFile {
            name,
            content_length,
            last_modified,
            content_md5,
            creation_time,
        }
    }
}

impl From<Blob> for BlobFile {
    fn from(entry: Blob) -> Self {
        BlobFile {
            name: entry.name,
            content_length: entry.properties.content_length as usize,
            last_modified: entry.properties.last_modified,
            content_md5: entry.properties.content_md5,
            creation_time: entry.properties.creation_time,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename = "EnumerationResults")]
pub struct ContainerEnumerationResults {
    #[serde(rename = "@ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "MaxResults", default)]
    pub max_results: i32,
    #[serde(rename = "Containers", default)]
    pub containers: ContainersList,
    #[serde(rename = "NextMarker", default)]
    pub next_marker: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub struct ContainersList {
    #[serde(rename = "Container", default)]
    pub container: Vec<Container>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Container {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: ContainerProps,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ContainerProps {
    #[serde(rename = "Last-Modified")]
    pub last_modified: String,
    #[serde(rename = "Etag")]
    pub etag: String,
    #[serde(rename = "PublicAccess", default)]
    pub public_access: String,
}
