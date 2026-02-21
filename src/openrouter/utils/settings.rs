use crate::openrouter::utils::types::Settings;
use arc_swap::ArcSwap;
use std::sync::{Arc, LazyLock};

static SETTINGS: LazyLock<ArcSwap<Settings>> =
    LazyLock::new(|| ArcSwap::from_pointee(Settings::load_from_disk()));

pub fn settings() -> Arc<Settings> {
    SETTINGS.load_full()
}

fn update_settings(mutator: impl FnOnce(&mut Settings)) {
    let current = SETTINGS.load_full();
    let mut next = (*current).clone();

    mutator(&mut next);

    next.save_to_disk();

    SETTINGS.store(Arc::new(next));
}

pub fn set_model(model: impl Into<String>) {
    update_settings(|s| s.model = model.into());
}

pub fn set_prompt<T>(file_name: Option<T>)
where
    T: Into<String>,
{
    update_settings(|s| s.prompt = file_name.map(|n| n.into()));
}
