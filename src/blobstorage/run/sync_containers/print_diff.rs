use crate::{
    blobstorage::types::{BlobFile, FileDiff, LocalFile},
    shared::terminal::codes::*,
};
use std::process;

pub fn print_diff(diff: &FileDiff) {
    print_count("Local files", diff.local_files_count.to_string().as_str());
    print_count("Remote files", diff.remote_files_count.to_string().as_str());
    print_count(
        "Changed files",
        diff.changed_files.len().to_string().as_str(),
    );

    if !diff.duplicate_files.is_empty() {
        print_count(
            "Duplicated files",
            diff.duplicate_files.len().to_string().as_str(),
        );

        for (file1, file2) in diff.duplicate_files.values() {
            println!("{TEXT_COLOR}{}{RESET_COLOR}", file1.name,);

            println!("{TEXT_COLOR}{}{RESET_COLOR}", file2.name,);
        }

        process::exit(1);
    }

    if !diff.new_files.is_empty() {
        println!("\n{WARNING_COLOR}New files:{RESET_COLOR}\n");

        for file in diff.new_files.values() {
            print_file(Some(file), None);
        }
    }

    if !diff.changed_files.is_empty() {
        println!("\n{INFO_COLOR}Changed files:{RESET_COLOR}\n",);

        for (local, remote) in diff.changed_files.values() {
            print_file(Some(local), Some(remote));
        }
    }

    if !diff.sync_available() {
        println!("{SUCCESS_COLOR}Container synced.{RESET_COLOR}",);
    }

    if !diff.deleted_files.is_empty() {
        println!("\n{DANGER_COLOR}Deleted files:{RESET_COLOR}\n",);

        for file in diff.deleted_files.values() {
            print_file(None, Some(file))
        }
    }
}

fn print_file(local: Option<&LocalFile>, remote: Option<&BlobFile>) {
    let (content_length, last_modified) = match (local, remote) {
        (Some(l), Some(r)) => {
            println!(
                "{TEXT_COLOR}Name:      {TEXT_HIGHLIGHTED_COLOR}{}{TEXT_COLOR} -> {TEXT_HIGHLIGHTED_COLOR}{}{RESET_COLOR}",
                r.name, l.name
            );

            (&l.content_length, &l.last_modified)
        }
        (Some(l), None) => {
            println!(
                "{TEXT_COLOR}Name:      {TEXT_HIGHLIGHTED_COLOR}{}{TEXT_COLOR}{RESET_COLOR}",
                l.name,
            );

            (&l.content_length, &l.last_modified)
        }
        (None, Some(r)) => {
            println!(
                "{TEXT_COLOR}Name:      {TEXT_HIGHLIGHTED_COLOR}{}{TEXT_COLOR}{RESET_COLOR}",
                r.name
            );

            (&r.content_length, &r.last_modified)
        }
        (None, None) => {
            unreachable!();
        }
    };

    println!(
        "{TEXT_COLOR}Size:      {TEXT_HIGHLIGHTED_FADED_COLOR}{} kb{RESET_COLOR}",
        content_length / 1024,
    );

    println!(
        "{TEXT_COLOR}Modified:  {SUCCESS_COLOR}{}{RESET_COLOR}\n",
        last_modified,
    );
}

fn print_count(title: &str, value: &str) {
    println!("{TEXT_COLOR}{}:  {TEXT_HIGHLIGHTED_COLOR}{}{RESET_COLOR}", title, value,);
}
