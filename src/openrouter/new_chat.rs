use crate::data;

pub fn run() {
    let data = data::get_app_data().expect("Faild to get app data.");

    println!("{}", data.api_key);
}
