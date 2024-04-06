#![feature(std_internals)]

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::os::unix::net::UnixListener;
use std::thread;



// Component: Represents a log message
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct LogMessage {
    timestamp: String,
    level: LogLevel,
    message: String,
}

// Enum for log levels
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

// System: Responsible for processing and outputting log messages
struct Logger {
    logs: Arc<Mutex<HashMap<LogLevel, Vec<LogMessage>>>>,
    log_file: File,
}

impl Logger {
    fn new(log_file_path: &str) -> Self {
        Logger {
            logs: Arc::new(Mutex::new(HashMap::new())),
            log_file: OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file_path)
                .unwrap(),
        }
    }

    fn log(&mut self, level: LogLevel, message: String) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_message = LogMessage {
            timestamp: timestamp.clone(),
            level,
            message: message.clone(),
        };

        // Write to file
        writeln!(&mut self.log_file, "[{}] [{:?}] {}", timestamp, level, message).unwrap();

        // Store in memory
        let mut logs = self.logs.lock().unwrap();
        logs.entry(level).or_insert(Vec::new()).push(log_message);
    }

    fn search(&self, query: &str) -> Vec<LogMessage> {
        let logs = self.logs.lock().unwrap();
        logs.iter()
            .flat_map(|(_, messages)| messages.iter())
            .filter(|message| message.message.contains(query))
            .cloned()
            .collect()
    }
}

fn main() {
    let mut logger = Logger::new("app.log");
    logger.log(LogLevel::Info, "This is an info message".to_string());
    logger.log(LogLevel::Warning, "This is a warning message".to_string());
    logger.log(LogLevel::Error, "This is an error message".to_string());

    let search_results = logger.search("warning");
    println!("Search results for 'warning': {:?}", search_results);
}

fn listener() {
    let listener = UnixListener::bind("/tmp/rustylogger").unwrap();
    println!("Listening for incoming logs...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let mut reader = BufReader::new(stream);
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();
        let log_message: LogMessage = serde_json::from_str(&buffer).unwrap();
        println!("Received log message: {:?}", log_message);
    }
}