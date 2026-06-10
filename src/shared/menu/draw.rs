use crate::shared::terminal::{COLORS, COLORPALETTE};
use std::{io::Write, iter};

pub fn write_headers<H, S>(header: H, subheaders: Vec<S>)
where
    H: AsRef<str>,
    S: AsRef<str>,
{
    let (header, left_line, right_line) = {
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

        let pad_left: String = iter::repeat_n("─", pad_left_len).collect();
        let pad_right: String = iter::repeat_n("─", pad_right_len).collect();
        (header_str, pad_left, pad_right)
    };

    println!(
        "{menu_color}{left_line} {header} {right_line}{reset}",
        menu_color = COLORPALETTE.Primary,
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
