use std::process::exit;
use windows_sys::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};

use crate::{init, input};
use std::io::{self, Write};

struct Opt {
    index: i32,
    text: String,
}

pub fn run(
    header: Option<String>,
    subheaders: Option<Vec<String>>,
    options: Vec<String>,
    exit_option: Option<String>,
    width: usize,
) {
    let stdin = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    init::init_console(stdout);

    let mut combined_opts: Vec<Opt> = options
        .into_iter()
        .enumerate()
        .map(|(index, text)| Opt {
            index: index as i32,
            text: text,
        })
        .collect();

    if let Some(exit_text) = exit_option {
        combined_opts.push(Opt {
            index: -1,
            text: exit_text,
        })
    }

    let mut menu_len = combined_opts.len();

    if let Some(header) = header {
        menu_len += 1;

        println!("\x1b[0;93m========== {} ==========\x1b[0m\n", header);
    }

    if let Some(subheaders) = subheaders {
        menu_len += subheaders.len();

        subheaders
            .into_iter()
            .for_each(|subheader| println!("\x1b[0;93m{}\x1b[0m", subheader));
    }

    let mut current_index = 0;
    let mut startRow = 0;

    render_menu(&combined_opts, current_index, width);

    loop {
        let key = input::read_key_blocking(stdin);

        // 👇 Restore to start of menu and clear everything below
        print!("\x1b[u"); // restore cursor to saved position
        print!("\x1b[J"); // clear from cursor down
        io::stdout().flush().unwrap();

        print!("\x1b[{}A\r", combined_opts.len());

        if let Some(ch) = key.ch {
            match ch {
                'q' | 'h' => exit(-1),

                'j' => {
                    if current_index != combined_opts.len() - 1 {
                        current_index += 1
                    }
                }
                'k' => {
                    if current_index != 0 {
                        current_index -= 1
                    }
                }

                'l' => {
                    clear_menu(menu_len, combined_opts.len());

                    exit(combined_opts[current_index].index as i32)
                }

                _ => continue,
            }
        }

        render_menu(&combined_opts, current_index, width);
    }
}

fn render_menu(options: &Vec<Opt>, current: usize, width: usize) {
    let content_width = width.saturating_sub(2);

    for i in 0..options.len() {
        if i == current {
            println!(
                "\x1b[0;93m> {: <num$}\x1b[0m",
                options[i].text,
                num = content_width
            );
        } else {
            println!("  {: <num$}", options[i].text, num = content_width);
        }
    }
}

fn clear_menu(menu_len: usize, options_len: usize) {
    println!("\x1b[{}A", menu_len - options_len + 1);
    println!("\x1b[0J");
}
