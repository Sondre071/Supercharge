use crate::openrouter;
use crate::shared::{menu, terminal};

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
            None,
        )
        .unwrap()
        .to_owned()
    };

    utils::set_model(&model);

    println!(
        "{yellow}Model set to: {white}{}{reset}",
        &model,
        yellow = COLORS.Yellow,
        white = COLORS.White,
        reset = COLORS.Reset
    )
}
