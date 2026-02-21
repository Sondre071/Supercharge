use std::{path::PathBuf, sync::OnceLock};

static DATA_PATH: OnceLock<PathBuf> = OnceLock::new();
static SCRIPTS_PATH: OnceLock<PathBuf> = OnceLock::new();

static OPENROUTER_PROMPTS_PATH: OnceLock<PathBuf> = OnceLock::new();
static OPENROUTER_SETTINGS_PATH: OnceLock<PathBuf> = OnceLock::new();

static BLOBSTORAGE_SETTINGS_PATH: OnceLock<PathBuf> = OnceLock::new();
static BLOBSTORAGE_CACHE_PATH: OnceLock<PathBuf> = OnceLock::new();

static SNIPPETS_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn data_dir() -> &'static PathBuf {
    DATA_PATH.get_or_init(|| {
        let mut path = std::env::home_dir().expect("Failed to fetch user home directory.");
        path.push(".supercharge");
        path.push("data");

        path
    })
}

pub fn scripts_dir() -> &'static PathBuf {
    SCRIPTS_PATH.get_or_init(|| {
        let mut path = std::env::home_dir().expect("Failed to fetch user home directory.");
        path.push(".supercharge");
        path.push("scripts");

        path
    })
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

pub fn openrouter_settings_path() -> &'static PathBuf {
    OPENROUTER_SETTINGS_PATH.get_or_init(|| {
        let mut path = data_dir().clone();
        path.push("openrouter.json");

        path
    })
}

pub fn blobstorage_settings_path() -> &'static PathBuf {
    BLOBSTORAGE_SETTINGS_PATH.get_or_init(|| {
        let mut path = data_dir().clone();
        path.push("blobstorage.json");

        path
    })
}

pub fn blobstorage_cache_path() -> PathBuf {
    BLOBSTORAGE_CACHE_PATH
        .get_or_init(|| {
            let mut path = data_dir().clone();
            path.push("blobstorage");
            path.push("cache");

            path
        })
        .clone()
}

pub fn snippets_path() -> PathBuf {
    SNIPPETS_PATH
        .get_or_init(|| {
            let mut path = data_dir().clone();
            path.push("snippets");

            path
        })
        .clone()
}
