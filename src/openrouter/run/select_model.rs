use crate::{
    openrouter::{api, utils::settings},
    shared::{
        menu::{self, Cursor, NONE},
        terminal::COLORS,
    },
};

pub fn select_model() {
    let model = {
        let models = api::fetch_models();

        let Some((prompt, _)) = menu::run(&mut Cursor::new(
            "Select model",
            NONE,
            models.iter().map(|m| m.as_str()).collect(),
        )) else {
            return;
        };

        prompt
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
