use crate::shared::{menu, statics, terminal::COLORS};

use std::io::{Read, Write};
use std::time::SystemTime;
use std::{fs, process};

pub fn add_snippet() {
    let temp_file_path = {
        let mut file_path = statics::snippets_path();

        let temp_file_name = format!(
            "{}.txt",
            SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis()
        );

        file_path.push(temp_file_name);

        file_path
    };

    let _ = process::Command::new("nvim")
        .args([&temp_file_path])
        .spawn()
        .expect("Failed to open neovim.")
        .wait();

    if fs::exists(&temp_file_path).unwrap() {
        menu::write_headers("New snippet", Some(&vec!["Enter name", ""]));

        let new_name = menu::read_line("File name: ").trim().to_owned();

        menu::clear_menu(4);

        let mut buffer = String::new();
        let mut file = fs::File::open(temp_file_path).unwrap();

        file.read_to_string(&mut buffer).unwrap();

        let new_path = {
            let mut pathbuf = statics::snippets_path();
            pathbuf.push(format!("{}.txt", new_name));

            pathbuf
        };

        let mut new_file = fs::File::create(new_path).unwrap();

        new_file.write_all(buffer.as_bytes()).unwrap();

        println!(
            "{yellow}Snippet {white}{}.txt {yellow}created!\n",
            new_name,
            white = COLORS.White,
            yellow = COLORS.Yellow
        );
    }
}
