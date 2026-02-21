use crate::{
    openrouter::{api, utils::settings},
    shared::{
        menu::{self, Cursor, NONE},
        terminal::COLORS,
    },
};

pub fn select_model() {
    let (model, _) = {
        let models = api::fetch_models();

        menu::run(&mut Cursor::new(
            "Select model",
            NONE,
            models.iter().map(|m| m.as_str()).collect(),
        ))
        .unwrap()
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
