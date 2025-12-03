use crate::data;
use crate::menu;

pub fn run() {
    let data = data::get_app_data();

    menu::r#loop::run("Hello!", None, vec!["", ""]);


    println!("{}", data.api_key);
}
