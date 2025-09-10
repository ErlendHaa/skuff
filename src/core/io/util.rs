pub mod json {
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    use crate::Error;

    pub fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, Error> {
        let text = std::fs::read_to_string(path)
            .map_err(|err| Error::FailedToReadFile(err.to_string()))?;

        let doc: T = serde_json::from_str(&text)
            .map_err(|_| Error::DeserializeFailed(path.display().to_string()))?;

        Ok(doc)
    }

    pub fn write<T: serde::Serialize>(path: &Path, content: &T) -> Result<(), Error> {
        let file = File::create(path).map_err(|err| Error::FailedToOpenFile(err.to_string()))?;

        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &content)
            .map_err(|err| Error::FailedToWriteFile(err.to_string()))?;

        Ok(())
    }
}

pub mod text {
    use std::path::Path;

    use crate::Error;

    pub fn read(path: &Path) -> Result<String, Error> {
        std::fs::read_to_string(path).map_err(|err| Error::FailedToReadFile(err.to_string()))
    }

    pub fn write(path: &Path, contents: &str) -> Result<(), Error> {
        std::fs::write(path, contents).map_err(|err| Error::FailedToWriteFile(err.to_string()))
    }
}

pub mod dirs {
    pub fn dirs(path: &std::path::Path) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
        let mut dirs = vec![];

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            }
        }

        Ok(dirs)
    }
}
