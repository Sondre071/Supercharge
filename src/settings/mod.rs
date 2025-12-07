use crate::binary;
use crate::data;
use crate::menu;

pub fn run() {
    if let Some(result) = menu::r#loop::run("Settings", None, vec!["Select model", "Back"]) {
        match result {
            "Select model" => select_model(),
            _ => {}
        }
    }
}

fn select_model() {
    let data = data::get_openrouter_data();

    let args = vec!["--api-key".to_string(), data.api_key.clone()];

    let mut binary_path = std::env::current_exe().unwrap();
    binary_path.pop();
    binary_path.push("bin");
    binary_path.push("fetch_models.exe");

    match binary::run_and_collect_lines(binary_path.to_str().unwrap(), args) {
        Ok(models) => {
            let mods: Vec<&str> = models.iter().map(|s| s.as_str()).collect();

            if let Some(result) = menu::r#loop::run("Select model", None, mods) {
                match result {
                    _ => {}
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to get models: {}", e);
        }
    }
}
