use crate::menu;

mod r#loop;

pub fn run() {
    if !has_ffmpeg() {
        eprintln!("ffmpeg missing from path.");
        return;
    }

    let header = "Choose file type";
    let items = vec!["jpg", "png", "Back"];

    let choice = menu::r#loop::run(header, None, items).unwrap();

    match choice {
        "Back" => {}
        _ => r#loop::run(choice),
    }
}

fn has_ffmpeg() -> bool {
    let has_ffmpeg = std::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    return has_ffmpeg;
}
