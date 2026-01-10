use crate::blobstorage;
use crate::shared::terminal;

use blobstorage::types::{BlobFile, FileDiff, LocalFile};
use std::process;
use terminal::COLORS;

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
            println!(
                "{white}{}{reset}",
                file1.name,
                white = COLORS.White,
                reset = COLORS.Reset
            );

            println!(
                "{white}{}{reset}",
                file2.name,
                white = COLORS.White,
                reset = COLORS.Reset
            );
        }

        process::exit(1);
    }

    if !diff.new_files.is_empty() {
        println!(
            "\n{cyan}New files:{reset}\n",
            cyan = COLORS.Cyan,
            reset = COLORS.Reset
        );

        for file in diff.new_files.values() {
            print_file(file, None);
        }
    }

    if !diff.changed_files.is_empty() {
        println!(
            "\n{cyan}Changed files:{reset}\n",
            cyan = COLORS.Cyan,
            reset = COLORS.Reset
        );

        for (local, remote) in diff.changed_files.values() {
            print_file(local, Some(remote));
        }
    }

    if !diff.sync_available() {
        println!(
            "{green}Container synced.{reset}",
            green = COLORS.Green,
            reset = COLORS.Reset
        );
    }
}

fn print_file(local: &LocalFile, remote: Option<&BlobFile>) {
    if let Some(r) = remote {
        println!(
            "{yellow}Name:      {white}{}{yellow} -> {white}{}{reset}",
            r.name,
            local.name,
            yellow = COLORS.Yellow,
            white = COLORS.White,
            reset = COLORS.Reset
        );
    } else {
        println!(
            "{yellow}Name:      {white}{}{yellow}{reset}",
            local.name,
            yellow = COLORS.Yellow,
            white = COLORS.White,
            reset = COLORS.Reset
        );
    }

    println!(
        "{yellow}Size:      {gray}{} kb{reset}",
        local.content_length / 1024,
        yellow = COLORS.Yellow,
        gray = COLORS.Gray,
        reset = COLORS.Reset
    );

    println!(
        "{yellow}Modified:  {green}{}{reset}\n",
        local.last_modified,
        yellow = COLORS.Yellow,
        green = COLORS.Green,
        reset = COLORS.Reset
    );
}

fn print_count(title: &str, value: &str) {
    println!(
        "{yellow}{}:  {white}{}{reset}",
        title,
        value,
        yellow = COLORS.Yellow,
        white = COLORS.White,
        reset = COLORS.Reset
    );
}
