use std::path::PathBuf;

lazy_static::lazy_static! {
    pub static ref DEFAULT_STORAGE_PATH: PathBuf = {
        let home = std::env::var("HOME")
            .expect("HOME env var not set");
        PathBuf::from(home).join(".local/share/skuff")
    };
}

pub struct StorageLayout {
    pub root: PathBuf,
}

impl StorageLayout {
    pub fn coalesce(explicit: Option<PathBuf>, settings: Option<PathBuf>) -> Self {
        let path = explicit
            .or(settings)
            .unwrap_or(DEFAULT_STORAGE_PATH.clone());

        Self { root: path }
    }

    pub fn stream_path(&self, stream: &str) -> PathBuf {
        self.root.join(format!("streams/{}/stream.json", stream))
    }

    pub fn streams_path(&self) -> PathBuf {
        self.root.join("streams")
    }

    pub fn config_path(&self, stream: &str) -> PathBuf {
        self.root.join(format!("streams/{}/config.json", stream))
    }

    pub fn current_stream_path(&self) -> PathBuf {
        self.root.join("CURRENT_STREAM")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn coalesce_explicit_wins() {
        let explicit = Some(PathBuf::from("/explicit/storage"));
        let settings = Some(PathBuf::from("/settings/storage"));

        let layout = StorageLayout::coalesce(explicit, settings);

        assert_eq!(layout.root, PathBuf::from("/explicit/storage"));
    }

    #[test]
    fn coalesce_settings_wins() {
        let explicit = None;
        let settings = Some(PathBuf::from("/settings/storage"));

        let layout = StorageLayout::coalesce(explicit, settings);

        assert_eq!(layout.root, PathBuf::from("/settings/storage"));
    }

    #[test]
    fn coalesce_default() {
        let explicit = None;
        let settings = None;

        let layout = StorageLayout::coalesce(explicit, settings);

        assert_eq!(layout.root, *DEFAULT_STORAGE_PATH);
    }
}
