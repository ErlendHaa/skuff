/// Layout module
///
/// This module defines the storage and config layouts. That is, where and how files are stored on
/// disk.
mod settings;
mod storage;

pub use settings::*;
pub use storage::*;
