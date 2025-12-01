use crate::menu::r#loop;

mod new_chat;

pub fn run() {
    let options = vec!["New chat", "Settings", "Back"];

    if let Some(result) = r#loop::run("OpenRouter", None, options) {
        match result {
            "New chat" => new_chat::run(),
            _ => {}
        }

        println!("{}", result);
    }
}
