use std::collections::HashMap;
use serde::Deserialize;
use smallvec::SmallVec;
use crate::Log;

// Components
#[derive(Deserialize)]
pub struct Timer {
    pub(crate) duration: u64,
    pub(crate) elapsed: u64,
}



#[derive(Deserialize)]
pub struct Config {
    pub(crate) log_file_path: String,
    pub(crate) max_log_size: u64, // in bytes
    pub(crate) rotation_count: usize,
}

#[derive(Deserialize)]
pub struct Timestamp(pub String);

#[derive(Deserialize)]
pub struct Level(pub LogLevel);

#[derive(Deserialize)]
pub struct Message(pub String);

// World - Everything in the log
pub struct World {
    pub(crate) entities: SmallVec<[Log; 16]>,
    pub(crate) timestamps: HashMap<Log, Timestamp>,
    pub(crate) levels: HashMap<Log, Level>,
    pub(crate) messages: HashMap<Log, Message>,
    pub(crate) next_log_id: Log,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}
