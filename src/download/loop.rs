use regex::Regex;
use std::io::{Error, ErrorKind};

use crate::menu;

pub fn run(extension: &str) {
    loop {
        let url = clipboard_win::get_clipboard_string().expect("Failed to get clipboard.");

        let header = "Download?";

        let options: Vec<&str> = vec!["Yes", "Refresh"];

        let choice = menu::r#loop::run(&header, Some(vec![url.as_str(), ""]), options)
            .expect("Failed to say yes or no.");

        match choice {
            "Yes" => {
                let name = parse_file_name(&url, extension);
                let save_path = select_save_path(&name);
                let status = download_file(&url, &save_path);

                if status.is_ok() {
                    println!("\x1b[0;32mDownloaded: {}\x1b[0m\n", &name)
                } else {
                    eprintln!("\x1b[0;93mFailed to download: {}\x1b[0m", &name)
                }
            }
            _ => {}
        }
    }
}

fn select_save_path(file_name: &str) -> String {
    let mut save_path = rfd::FileDialog::new()
        .pick_folder()
        .expect("Failed to select save path.");
    save_path.push(file_name);

    save_path
        .to_str()
        .expect("Failed to parse save path to string.")
        .to_owned()
}

fn download_file(url: &str, save_path: &str) -> Result<(), Error> {
    let status = std::process::Command::new("ffmpeg")
        .arg("-loglevel")
        .arg("error")
        .arg("-i")
        .arg(url)
        .arg("-frames:v")
        .arg("1")
        .arg("-q:v")
        .arg("1")
        .arg(save_path)
        .output()?;

    if !status.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            "ffmpeg download failed.",
        ));
    }

    Ok(())
}

fn parse_file_name(url: &str, extension: &str) -> String {
    let rx = Regex::new(r"[a-zA-Z0-9%_]+").unwrap();

    let raw_name = {
        let arr: Vec<&str> = url.split('/').collect();

        arr[arr.len().saturating_sub(1)]
    };

    let str_match = rx
        .find(raw_name)
        .expect("Failed to match clipboard to regex.");

    format!("{}.{}", str_match.as_str(), extension)
}

#[test]
fn test() {
    println!("testing regex");
    println!();

    let url = "https://pbs.twimg.com/media/G7v4ZdZXcAAb_a%30Pe8?format=jpg&name=large";

    let rx = Regex::new(r"[a-zA-Z0-9%_]+").unwrap();

    let raw_name = {
        let arr: Vec<&str> = url.split('/').collect();

        arr[arr.len() - 1]
    };

    let str_match = rx
        .find(&raw_name)
        .expect("Failed to match clipboard to regex.");

    println!("{}", str_match.as_str());
}
