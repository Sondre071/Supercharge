use crate::shared::menu;

pub fn main() {
    let result = menu::run(
        "OpenRouter",
        None,
        vec!["New chat", "Settings", "Back"],
        None,
    )
    .unwrap();

    match result {
        "New chat" => super::new_chat(),
        "Settings" => super::settings(),
        _ => {}
    }
}
