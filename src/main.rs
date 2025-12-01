mod menu;

fn main() {
    let mut current = Some(menu::MenuId::Home);

    while let Some(current_id) = current {
        current = menu::r#loop::run(current_id);
    }
}
