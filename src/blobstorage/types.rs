use crate::utils::date;
use super::api::types::Blob;

use base64::{Engine as _, engine::general_purpose};
use std::fs;
use std::path::PathBuf;
use walkdir::{self, DirEntry};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LocalFile {
    pub name: String,
    pub content_length: usize,
    pub last_modified: String,
    pub creation_time: String,
    pub content_md5: String,
    pub path: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BlobFile {
    pub name: String,
    pub content_length: usize,
    pub last_modified: String,
    pub creation_time: String,
    pub content_md5: String,
}

impl From<DirEntry> for LocalFile {
    fn from(entry: DirEntry) -> Self {
        let path = entry.path();
        let metadata = entry.metadata().expect("Failed to parse metadata.");

        let name = entry.file_name().to_string_lossy().into_owned();

        if name.contains(".jif") {
            panic!("Found a .jif file. Get rid of it.")
        }

        let content_length = metadata.len() as usize;

        let last_modified = date::format_date(metadata.modified().unwrap());

        let creation_time = date::format_date(metadata.created().unwrap());

        let bytes = fs::read(path).expect("Failed to parse file content.");

        let digest = md5::compute(&bytes);
        let content_md5 = general_purpose::STANDARD.encode(digest.0);

        LocalFile {
            name,
            content_length,
            last_modified,
            content_md5,
            creation_time,
            path: path.to_owned(),
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