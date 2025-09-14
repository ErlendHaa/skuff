use super::Id;

#[derive(Debug)]
pub enum Error {
    StreamAlreadyExists(String),
    StreamCreationFailed(String),
    StorageInitializationFailed(String),
    StreamDoesNotExist(String),
    EntityDoesNotExist(Id),
    EntityIdExists(Id),
    FailedToWriteFile(String),
    FailedToOpenFile(String),
    FailedToReadFile(String),
    FailedToReadDir(String),
    CustomConfigNotFound(String),
    NoStreamSet,
    SerializeFailed(String),
    DeserializeFailed(String),
}
