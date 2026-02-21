use crate::shared::menu::{self, *};

use crate::openrouter::utils::{get_prompts, settings};

pub fn select_prompt() {
    let prompts = get_prompts();

    let result = menu::run(&mut Cursor::new(
        "Select prompt",
        Some(vec![""]),
        prompts.iter().map(|p| &p.name).collect(),
    ));
    
    let Some((prompt_name, _)) = result else {
        return;
    };

    settings::set_prompt(Some(prompt_name));
}
