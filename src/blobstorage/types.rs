use crate::api::types::Blob;

use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use time::{OffsetDateTime, format_description};

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

        if name.contains(".jif") {
            panic!("Found a .jif file. Get rid of it.")
        }

        let content_length = metadata.len() as usize;

        let last_modified = format_date(metadata.modified().unwrap());

        let creation_time = format_date(metadata.created().unwrap());

        let bytes = fs::read(path).expect("Failed to parse file content.");

        let digest = md5::compute(&bytes);
        let content_md5 = general_purpose::STANDARD.encode(digest.0);

        BlobFile {
            name,
            content_length,
            last_modified,
            content_md5,
            creation_time,
        }
    }
}

fn format_date(t: SystemTime) -> String {
    let fmt =
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    OffsetDateTime::from(t).format(&fmt).unwrap()
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
