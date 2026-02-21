use std::{path::PathBuf, sync::OnceLock};

static DATA_PATH: OnceLock<PathBuf> = OnceLock::new();
static SCRIPTS_PATH: OnceLock<PathBuf> = OnceLock::new();

static OPENROUTER_PROMPTS_PATH: OnceLock<PathBuf> = OnceLock::new();
static OPENROUTER_SETTINGS_PATH: OnceLock<PathBuf> = OnceLock::new();

static BLOBSTORAGE_SETTINGS_PATH: OnceLock<PathBuf> = OnceLock::new();
static BLOBSTORAGE_CACHE_PATH: OnceLock<PathBuf> = OnceLock::new();

static SNIPPETS_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn data_dir() -> PathBuf {
    DATA_PATH
        .get_or_init(|| {
            let mut path = std::env::home_dir().expect("Failed to fetch user home directory.");
            path.push("AppData");
            path.push("Local");
            path.push("Supercharge");

            path
        })
        .to_owned()
}

pub fn scripts_dir() -> PathBuf {
    SCRIPTS_PATH
        .get_or_init(|| {
            let mut path = data_dir().to_owned();
            path.push("scripts");

            path
        })
        .to_owned()
}

pub fn prompts_dir() -> PathBuf {
    OPENROUTER_PROMPTS_PATH
        .get_or_init(|| {
            let mut path = data_dir().to_owned();
            path.push("prompts");

            path
        })
        .to_owned()
}

pub fn openrouter_settings_path() -> PathBuf {
    OPENROUTER_SETTINGS_PATH
        .get_or_init(|| {
            let mut path = data_dir().clone();
            path.push("openrouter.json");

            path
        })
        .to_owned()
}

pub fn blobstorage_settings_path() -> PathBuf {
    BLOBSTORAGE_SETTINGS_PATH
        .get_or_init(|| {
            let mut path = data_dir().clone();
            path.push("blobstorage.json");

            path
        })
        .to_owned()
}

pub fn blobstorage_cache_path() -> PathBuf {
    BLOBSTORAGE_CACHE_PATH
        .get_or_init(|| {
            let mut path = data_dir().clone();
            path.push("blobstorage");
            path.push("cache");

            path
        })
        .to_owned()
}

pub fn snippets_path() -> PathBuf {
    SNIPPETS_PATH
        .get_or_init(|| {
            let mut path = data_dir().clone();
            path.push("snippets");

            path
        })
        .to_owned()
}
