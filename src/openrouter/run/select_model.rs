use crate::openrouter;
use crate::shared::{menu::{self, Cursor, NONE}, terminal};

use openrouter::api;
use openrouter::utils;
use terminal::COLORS;

pub fn select_model() {
    let (model, _) = {
        let data = utils::get_local_data();
        let models = api::fetch_models(&data.api_key);

        menu::run(&mut Cursor::new(
            "Select model",
            NONE,
            models.iter().map(|m| m.as_str()).collect()
        )).unwrap()
    };

    utils::set_model(model.as_str());

    println!(
        "{yellow}Model set to: {white}{}{reset}",
        &model,
        yellow = COLORS.Yellow,
        white = COLORS.White,
        reset = COLORS.Reset
    )
}
