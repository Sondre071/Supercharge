use std::path::PathBuf;
use std::sync::OnceLock;

static DATA_PATH: OnceLock<PathBuf> = OnceLock::new();
static PROMPTS_PATH: OnceLock<PathBuf> = OnceLock::new();

static OPENROUTER_SETTINGS_PATH: OnceLock<PathBuf> = OnceLock::new();
static BLOBSTORAGE_SETTINGS_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn data_dir() -> &'static PathBuf {
    DATA_PATH.get_or_init(|| {
        let mut path = std::env::home_dir().expect("Failed to fetch user home directory.");
        path.push(".supercharge");
        path.push("data");

        path
    })
}

pub fn prompts_dir() -> &'static PathBuf {
    PROMPTS_PATH.get_or_init(|| {
        let mut path = data_dir().clone();
        path.push("prompts");

        path
    })
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