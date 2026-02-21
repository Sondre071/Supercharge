use crate::{
    openrouter::{api, utils::settings},
    shared::{
        menu::{self, Cursor, NONE},
        terminal::COLORS,
    },
};

pub fn select_model() {
    let Some((model, _)) = menu::run(&mut Cursor::new("Select model", NONE, api::fetch_models()))
    else {
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
