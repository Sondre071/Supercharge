use crate::{
    blobstorage::types::{BlobFile, FileDiff, LocalFile},
    shared::terminal::COLORS,
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
            print_file(Some(file), None);
        }
    }

    if !diff.changed_files.is_empty() {
        println!(
            "\n{cyan}Changed files:{reset}\n",
            cyan = COLORS.Cyan,
            reset = COLORS.Reset
        );

        for (local, remote) in diff.changed_files.values() {
            print_file(Some(local), Some(remote));
        }
    }

    if !diff.sync_available() {
        println!(
            "{green}Container synced.{reset}",
            green = COLORS.Green,
            reset = COLORS.Reset
        );
    }

    if !diff.deleted_files.is_empty() {
        println!(
            "\n{red}Deleted files:{reset}\n",
            red = COLORS.Red,
            reset = COLORS.Reset
        );

        for file in diff.deleted_files.values() {
            print_file(None, Some(file))
        }
    }
}

fn print_file(local: Option<&LocalFile>, remote: Option<&BlobFile>) {
    let (content_length, last_modified) = match (local, remote) {
        (Some(l), Some(r)) => {
            println!(
                "{yellow}Name:      {white}{}{yellow} -> {white}{}{reset}",
                r.name,
                l.name,
                yellow = COLORS.Yellow,
                white = COLORS.White,
                reset = COLORS.Reset
            );

            (&l.content_length, &l.last_modified)
        }
        (Some(l), None) => {
            println!(
                "{yellow}Name:      {white}{}{yellow}{reset}",
                l.name,
                yellow = COLORS.Yellow,
                white = COLORS.White,
                reset = COLORS.Reset
            );

            (&l.content_length, &l.last_modified)
        }
        (None, Some(r)) => {
            println!(
                "{yellow}Name:      {white}{}{yellow}{reset}",
                r.name,
                yellow = COLORS.Yellow,
                white = COLORS.White,
                reset = COLORS.Reset
            );

            (&r.content_length, &r.last_modified)
        }
        (None, None) => {
            unreachable!();
        }
    };

    println!(
        "{yellow}Size:      {gray}{} kb{reset}",
        content_length / 1024,
        yellow = COLORS.Yellow,
        gray = COLORS.Gray,
        reset = COLORS.Reset
    );

    println!(
        "{yellow}Modified:  {green}{}{reset}\n",
        last_modified,
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
