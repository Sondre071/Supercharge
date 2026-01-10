use crate::blobstorage;

use blobstorage::types::{LocalFile, BlobFile, FileDiff};

use std::collections::{HashMap, HashSet};

pub fn create_diff(
    mut local_files: HashMap<String, LocalFile>,
    mut remote_files: HashMap<String, BlobFile>,
) -> FileDiff {
    let mut diff = FileDiff::default();
    
    diff.local_files_count = local_files.len();
    diff.remote_files_count = remote_files.len();

    let unique_hashes: HashSet<String> = local_files
        .keys()
        .chain(remote_files.keys())
        .cloned()
        .collect();

    for hash in unique_hashes {
        match (local_files.remove(&hash), remote_files.remove(&hash)) {
            (Some(l), Some(r)) => {
                if l.name != r.name {
                    diff.changed_files.insert(hash, (l, r));
                }
            }
            (Some(l), None) => {
                diff.new_files.insert(hash, l);
            }
            (None, Some(r)) => {
                diff.deleted_files.insert(hash, r);
            }
            (None, None) => {
                unreachable!()
            }
        }
    }

    diff
}