use std::fs;

use crate::Config;
use crate::Error;
use crate::Event;
use crate::Stream;
use crate::layout::StorageLayout;

use super::util::*;

pub struct Storage {
    layout: StorageLayout,
}

impl Storage {
    pub fn new(layout: StorageLayout) -> Self {
        Self { layout }
    }

    pub fn current_stream(&self) -> Result<Option<String>, Error> {
        let path = self.layout.current_stream_path();

        if !path.exists() {
            return Ok(None);
        }

        let current = text::read(&path)?;

        Ok(Some(current))
    }

    pub fn set_current_stream(&self, stream: &str) -> Result<(), Error> {
        let path = self.layout.current_stream_path();

        if !path.exists() {
            return Err(Error::StreamDoesNotExist(stream.to_string()));
        }

        text::write(&path, stream)?;

        Ok(())
    }

    pub fn stream_exists(&self, stream: &str) -> Result<bool, Error> {
        let path = self.layout.stream_path(stream);

        Ok(path.exists())
    }

    pub fn stream_create(&self, stream: &str) -> Result<(), Error> {
        let path = self.layout.stream_path(stream);

        if path.exists() {
            return Err(Error::StreamAlreadyExists(stream.to_string()));
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| Error::StreamCreationFailed(err.to_string()))?;
        }

        let buf = Stream::new().to_buffer()?;

        fs::write(&path, buf).map_err(|err| Error::FailedToWriteFile(err.to_string()))?;

        Ok(())
    }

    pub fn stream(&self, stream_name: &Option<String>) -> Result<Stream, Error> {
        let stream_name = self.coalease_stream_name(&stream_name)?;

        let path = self.layout.stream_path(&stream_name);

        if !path.exists() {
            return Err(Error::StreamDoesNotExist(stream_name));
        }

        let buf = fs::read(&path).map_err(|err| Error::FailedToReadFile(err.to_string()))?;

        Stream::from_buffer(&buf)
    }

    pub fn stream_append(&self, event: Event, stream_name: &Option<String>) -> Result<(), Error> {
        let stream_name = self.coalease_stream_name(&stream_name)?;

        if !self.stream_exists(&stream_name)? {
            return Err(Error::StreamDoesNotExist(stream_name.to_string()));
        }

        let path = self.layout.stream_path(&stream_name);

        let buf = fs::read(&path).map_err(|err| Error::FailedToReadFile(err.to_string()))?;

        let mut stream = Stream::from_buffer(&buf)?;

        let _ = stream.push(event);

        let buf = stream.to_buffer()?;
        fs::write(&path, buf).map_err(|err| Error::FailedToWriteFile(err.to_string()))?;

        Ok(())
    }

    pub fn streams(&self) -> Result<Vec<String>, Error> {
        let path = self.layout.streams_path();

        if !path.exists() {
            return Ok(vec![]);
        }

        let streams = dirs::dirs(&path)
            .map_err(|err| Error::FailedToReadDir(err.to_string()))?
            .iter()
            .filter(|p| p.is_dir())
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect::<Vec<String>>();

        Ok(streams)
    }

    pub fn config_file(&self, stream_name: &Option<String>) -> Result<Option<Config>, Error> {
        let stream_name = self.coalease_stream_name(&stream_name)?;

        let path = self.layout.config_path(&stream_name);

        if !path.exists() {
            return Ok(None);
        }

        let config: Config = json::read(&path)?;

        Ok(Some(config))
    }

    fn coalease_stream_name(&self, stream_name: &Option<String>) -> Result<String, Error> {
        match stream_name {
            Some(s) => Ok(s.clone()),
            None => match self.current_stream()? {
                Some(s) => Ok(s),
                None => Err(Error::NoStreamSet),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn create_new_stream() {
        let env = TestEnv::new();
        let io = Storage::new(env.layout());

        io.stream_create("stream1").unwrap();

        let expected_path = env.root.join("streams/stream1/stream.json");
        assert!(expected_path.exists());
    }

    #[test]
    fn create_stream_fails_if_already_exists() {
        let env = TestEnv::new();
        let io = Storage::new(env.layout());

        // First create succeeds
        io.stream_create("stream1").unwrap();
        // Second create should fail
        let err = io.stream_create("stream1").unwrap_err();

        match err {
            Error::StreamAlreadyExists(s) => assert_eq!(s, "stream1"),
            other => panic!("expected StreamAlreadyExists, got {:?}", other),
        }
    }

    #[test]
    fn exists_returns_false_if_stream_missing() {
        let env = TestEnv::new();
        let io = Storage::new(env.layout());

        let result = io.stream_exists("stream1").unwrap();
        assert!(!result);
    }

    #[test]
    fn exists_returns_true_if_stream_exists() {
        let env = TestEnv::new();
        let io = Storage::new(env.layout());

        // Manually create the directory
        let stream_path = env.root.join("streams/stream1");
        std::fs::create_dir_all(&stream_path).unwrap();
        std::fs::File::create(stream_path.join("stream.json")).unwrap();

        let result = io.stream_exists("stream1").unwrap();
        assert!(result);
    }

    struct TestEnv {
        root: PathBuf,
        _temp: TempDir, // keep TempDir alive so it cleans up
    }

    impl TestEnv {
        fn new() -> Self {
            let temp = TempDir::new().expect("failed to create tempdir");
            let root = temp.path().to_path_buf();
            Self { root, _temp: temp }
        }

        fn layout(&self) -> StorageLayout {
            StorageLayout::coalesce(None, Some(self.root.clone()))
        }
    }
}
