use crate::menu;
use crate::openrouter;
use crate::terminal;

use openrouter::api;
use openrouter::utils;
use terminal::COLORS;

pub fn select_model() {
    let model = {
        let data = utils::get_local_data();

        let models = api::fetch_models(&data.api_key);

        menu::run(
            "Select model",
            Some(vec![]),
            models.iter().map(|m| m.as_str()).collect(),
        )
        .unwrap()
        .to_owned()
    };

    utils::set_model(&model);

    println!(
        "{}Model set to: {}{}{}",
        COLORS.Yellow, COLORS.White, &model, COLORS.Gray
    )
}
