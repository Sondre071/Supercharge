use crate::{
    openrouter::{
        api::types::InputMessage,
        utils::get_prompts,
        utils::settings,
    },
    shared::{menu, menu::Cursor, statics},
};
use std::iter::once;

pub fn select_prompt() -> Option<InputMessage> {
    let prompts = get_prompts();

    let current_prompt = settings().prompt.clone().unwrap_or("None".to_string());

    let file_name = menu::run(&mut Cursor::new(
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

    let Some(Some(prompt_name)) = file_name else {
        return None
    };
    
    let content = {
        let mut file_path = statics::prompts_dir();
        file_path.push(prompt_name);
        
        std::fs::read_to_string(file_path).expect("Failed to read prompt content.")
    };

    Some(InputMessage {
        role: "system".to_owned(),
        content,
    })
}
