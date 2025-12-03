use std::io;
use std::io::Write;

use crate::data;
use crate::menu;

pub fn run() {
    let data = data::get_app_data();

    menu::write_headers("New chat", Some(&vec![&data.model, ""]));

    loop {
        menu::read_line("You: ");

        println!("\ngood job!\n")
    }

    print!("End.");

    //println!("{}", data.api_key);
}
