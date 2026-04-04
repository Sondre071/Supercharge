use crate::{
    openrouter::utils::{get_prompts, settings},
    shared::menu::{self, *},
};
use std::iter::once;

pub fn select_prompt() {
    let prompts = get_prompts();

    let current_prompt = settings().prompt.clone().unwrap_or("None".to_string());

    let result = menu::run(&mut Cursor::new(
        "Select prompt",
        Some(vec![
            format!("Current prompt: {current_prompt}").as_str(),
            "",
        ]),
        once("None")
            .chain(prompts.iter().map(|p| p.name.as_str()))
            .collect(),
        None,
    ))
    .map(|(f, _)| if f == "None" { None } else { Some(f) });

    let Some(prompt_name) = result else {
        return;
    };

    settings::set_prompt(prompt_name);
}
