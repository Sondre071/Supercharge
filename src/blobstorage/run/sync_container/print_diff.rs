use crate::blobstorage;
use crate::terminal;

use terminal::COLORS;

use std::process;

use blobstorage::types::FileDiff;

pub fn print_diff(diff: &FileDiff) {
    println!(
        "{yellow}Local files:  {white}{}{reset}",
        diff.local_files_count,
        yellow = COLORS.Yellow,
        white = COLORS.White,
        reset = COLORS.Reset
    );

    println!(
        "{yellow}Remote files:  {white}{}{reset}",
        diff.remote_files_count,
        yellow = COLORS.Yellow,
        white = COLORS.White,
        reset = COLORS.Reset
    );

    if !diff.changed_files.is_empty() {
        println!(
            "{yellow}Updated files: {white}{}{reset}\n",
            diff.changed_files.len(),
            yellow = COLORS.Yellow,
            white = COLORS.White,
            reset = COLORS.Reset
        );
    }

    if !diff.duplicate_files.is_empty() {
        println!(
            "{red}Duplicate files: {white}{}{reset}\n",
            diff.duplicate_files.len(),
            red = COLORS.Red,
            white = COLORS.White,
            reset = COLORS.Reset
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

    for file in diff.new_files.values() {
        println!(
            "\n{cyan}New files:{reset}\n",
            cyan = COLORS.Cyan,
            reset = COLORS.Reset
        );

        println!(
            "{yellow}Name:      {white}{}{reset}",
            file.name,
            yellow = COLORS.Yellow,
            white = COLORS.White,
            reset = COLORS.Reset
        );

        println!(
            "{yellow}Size:      {gray}{} kb{reset}",
            file.content_length / 1024,
            yellow = COLORS.Yellow,
            gray = COLORS.Gray,
            reset = COLORS.Reset
        );

        println!(
            "{yellow}Modified:  {green}{}{reset}\n",
            file.last_modified,
            yellow = COLORS.Yellow,
            green = COLORS.Green,
            reset = COLORS.Reset
        );
    }

    if !diff.sync_available() {
        println!(
            "{green}Container synced.{reset}",
            green = COLORS.Green,
            reset = COLORS.Reset
        );
    }
}
