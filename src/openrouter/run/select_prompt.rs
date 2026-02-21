use crate::{
    openrouter::utils::{get_prompts, settings},
    shared::menu::{self, *},
};
use std::iter::once;

pub fn select_prompt() {
    let prompts = get_prompts();

    let result = menu::run(&mut Cursor::new(
        "Select prompt",
        Some(vec![""]),
        once("None")
            .chain(prompts.iter().map(|p| p.name.as_str()))
            .collect(),
    ))
    .map(|(f, _)| {
        if f == "None" {
            None
        } else {
            Some(f)
        }
    });
    
    let Some(prompt_name) = result else {
        return;
    };

    settings::set_prompt(prompt_name);
}
