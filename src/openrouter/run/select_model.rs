use crate::{
    openrouter::{api, utils::settings},
    shared::{
        menu::{self, Cursor},
        terminal::codes::*,
    },
};

pub fn select_model() {
    let settings = settings();

    let subheader = vec![
        format!("Current model: {}", settings.model.clone()),
        "".to_string(),
    ];

    let Some((model, _)) = menu::run(&mut Cursor::new(
        "Select model",
        Some(subheader),
        api::fetch_models(),
        None,
    )) else {
        return;
    };

    settings::set_model(&model);

    println!("{INFO_COLOR}Model set to: {TEXT_COLOR}{}{RESET_COLOR}", model);
}
