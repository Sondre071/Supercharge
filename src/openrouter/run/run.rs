use crate::menu;

pub fn run() {
    if let Some(result) = menu::run("OpenRouter", None, vec!["New chat", "Settings", "Back"]) {
        match result {
            "New chat" => super::new_chat(),
            "Settings" => super::settings(),
            _ => crate::main(),
        }
    } else {
        crate::main();
    }
}
