use std::io::Write;

mod r#loop;
mod cursor;
mod input;

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

        let pad_left: String = std::iter::repeat("=").take(pad_left_len).collect();
        let pad_right: String = std::iter::repeat("=").take(pad_right_len).collect();
        format!("\x1b[0;93m{} {} {}\x1b[0m", pad_left, header_str, pad_right)
    };

    println!("{header_text}");

    if subheaders.is_some() {
        for subheader in subheaders.unwrap().iter() {
            println!("\x1b[0;93m{}\x1b[0m", subheader)
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
