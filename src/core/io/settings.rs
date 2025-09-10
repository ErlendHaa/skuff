use std::path::PathBuf;

use super::util::*;
use crate::Config;
use crate::Error;
use crate::layout::DEFAULT_STORAGE_PATH;
use crate::layout::SettingsLayout;

pub struct Settings;

impl Settings {
    pub fn storage_path() -> Result<PathBuf, Error> {
        let filepath = SettingsLayout::current_storage();

        if !filepath.exists() {
            return Ok(DEFAULT_STORAGE_PATH.clone());
        }

        let path = text::read(&filepath)?;

        Ok(path.into())
    }

    pub fn config_file() -> Result<Option<Config>, Error> {
        let filepath = SettingsLayout::config_path();

        if !filepath.exists() {
            return Ok(None);
        }

        let config: Config = json::read(&filepath)?;

        Ok(Some(config))
    }
}
