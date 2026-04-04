use crate::{
    openrouter::{api, utils::settings},
    shared::{
        menu::{self, Cursor},
        terminal::COLORS,
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

    println!(
        "{yellow}Model set to: {white}{}{reset}",
        model,
        yellow = COLORS.Yellow,
        white = COLORS.White,
        reset = COLORS.Reset
    );
}
