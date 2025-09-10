use std::path::PathBuf;

pub struct SettingsLayout;

lazy_static::lazy_static! {
    pub static ref SETTINGS_DIRECTORY: PathBuf = {
        let home = std::env::var("HOME")
            .expect("HOME env var not set");
        PathBuf::from(home).join(".skuff")
    };
}

impl SettingsLayout {
    pub fn current_storage() -> PathBuf {
        SETTINGS_DIRECTORY.join("CURRENT_STORAGE")
    }

    pub fn config_path() -> PathBuf {
        SETTINGS_DIRECTORY.join("config.json")
    }
}
