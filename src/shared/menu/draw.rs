use crate::shared::terminal::COLORS;

use std::{io::Write, iter};

pub fn write_headers<H, S>(header: H, subheaders: Vec<S>)
where
    H: AsRef<str>,
    S: AsRef<str>,
{
    let header_text = {
        let width: usize = 30;

        // Truncate
        let mut header_str = header.as_ref().to_string();

        if header_str.chars().count() > width {
            let truncated: String = header_str.chars().take(width - 2).collect();
            header_str = format!("{truncated}..");
        }

        // Format
        let pad_left_len = (width.saturating_sub(header_str.chars().count()) - 2) / 2;
        let pad_right_len = width - pad_left_len - header_str.chars().count();

        let pad_left: String = iter::repeat_n("═", pad_left_len).collect();
        let pad_right: String = iter::repeat_n("═", pad_right_len).collect();
        format!("{} {} {}", pad_left, header_str, pad_right)
    };

    println!(
        "{yellow}{}{reset}",
        header_text,
        yellow = COLORS.Yellow,
        reset = COLORS.Reset
    );

    for subheader in subheaders.iter() {
        println!(
            "{yellow}{}{reset}",
            subheader.as_ref(),
            yellow = COLORS.Yellow,
            reset = COLORS.Reset
        )
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
