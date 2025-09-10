/// Format Module
///
/// This module defines the storage format for config files and data streams through datastructures
/// in rust. Both config and stream data are stored as JSON files on disk.
mod config;
mod stream;

pub use config::*;
pub use stream::*;
