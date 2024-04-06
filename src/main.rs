
mod components;

use chrono::Local;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use smallvec::SmallVec;
use crate::components::*;


// Log
type Log = u32;

impl World {
    fn new() -> Self {
        World {
            entities: SmallVec::new(),
            timestamps: HashMap::new(),
            levels: HashMap::new(),
            messages: HashMap::new(),
            next_log_id: 0,
        }
    }

    fn create_log(&mut self, timestamp: Timestamp, level: Level, message: Message) -> Log {
        let log_id = self.next_log_id;
        self.next_log_id += 1;
        self.entities.push(log_id);
        self.timestamps.insert(log_id, timestamp);
        self.levels.insert(log_id, level);
        self.messages.insert(log_id, message);
        log_id
    }
}

// Systems
async fn log_system(world: &mut World, level: LogLevel, message_text: String) {
    let timestamp = Timestamp(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
    let level = Level(level);
    let message = Message(message_text);
    world.create_log(timestamp, level, message);
}

// Main function and network listener
#[tokio::main]
async fn main() {
    let world = Arc::new(Mutex::new(World::new()));
    let listener_task = tokio::spawn(listener_system(world.clone()));

    {
        let mut world = world.lock().await;
        log_system(&mut world, LogLevel::Info, "This is an info message".to_string()).await;
        log_system(&mut world, LogLevel::Warning, "This is a warning message".to_string()).await;
        log_system(&mut world, LogLevel::Error, "This is an error message".to_string()).await;
    }

    let _ = listener_task.await;
}

async fn listener_system(world: Arc<Mutex<World>>) {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening for incoming logs...");


    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let world_clone = world.clone();
        tokio::spawn(async move {
            handle_client(socket, world_clone).await;
        });
    }
}

async fn handle_client(mut socket: tokio::net::TcpStream, _world: Arc<Mutex<World>>) {
    let mut reader = BufReader::new(&mut socket);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).await.unwrap() > 0 {
        // Log received from
        println!("Received log: {}", buffer.trim());

        // Write log to file
       OpenOptions::new()
            .create(true)
            .append(true);

        // Create a new log file if it doesn't exist
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("logs.txt") {
            let log_message = buffer.trim();
            if let Err(e) = writeln!(file, "{}", log_message) {
                eprintln!("Failed to write to file: {}", e);
            }
        }

        buffer.clear();
    }
}