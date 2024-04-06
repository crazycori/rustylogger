use serde::Deserialize;

// Components
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub(crate) log_file_path: String,
    pub(crate) max_log_size: u64, // in bytes
    pub(crate) rotation_count: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Timestamp(pub String);

#[derive(Debug, Deserialize, Clone)]
pub struct Level(pub LogLevel);

#[derive(Debug, Deserialize, Clone)]
pub struct Message(pub(crate) String);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}
