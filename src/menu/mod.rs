use crate::utils;

use std::io::Write;
use std::iter;
use utils::terminal;
use terminal::COLORS;

mod cursor;
mod r#loop;
pub use r#loop::run;

pub fn write_headers(header: &str, subheaders: Option<&Vec<&str>>) {
    let header_text = {
        let width: usize = 30;

        // Truncate
        let mut header_str = header.to_string();

        if header_str.chars().count() > width {
            let truncated: String = header_str.chars().take(width - 2).collect();
            header_str = format!("{truncated}..");
        }

        // Format
        let pad_left_len = (width.saturating_sub(header_str.chars().count()) - 2) / 2;
        let pad_right_len = width - pad_left_len - header_str.chars().count();

        let pad_left: String = iter::repeat_n("=", pad_left_len).collect();
        let pad_right: String = iter::repeat_n("=", pad_right_len).collect();
        format!("{} {} {}", pad_left, header_str, pad_right)
    };

    println!(
        "{yellow}{}{reset}",
        header_text,
        yellow = COLORS.Yellow,
        reset = COLORS.Reset
    );

    if let Some(items) = subheaders {
        for subheader in items.iter() {
            println!(
                "{yellow}{}{reset}",
                subheader,
                yellow = COLORS.Yellow,
                reset = COLORS.Reset
            )
        }
    }
}

pub fn clear_menu(height: usize) {
    print!("\x1b[{}A", height);
    print!("\x1b[0J");

    std::io::stdout().flush().unwrap();
}

pub fn read_line(instruction: &str) -> String {
    print!("{}", instruction);

    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Read-line failed.");

    input
}
