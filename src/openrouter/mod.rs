use crate::menu::r#loop;

mod new_chat;
mod types;

pub fn run() {
    if let Some(result) = r#loop::run("OpenRouter", None, vec!["New chat", "Back"]) {
        match result {
            "New chat" => new_chat::run(),
            _ => {}
        }

        println!("{}", result);
    }
}
