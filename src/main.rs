use chrono::Local;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use smallvec::SmallVec;

// Components
#[derive(Debug, Deserialize)]
struct Timestamp(String);

#[derive(Debug, Deserialize)]
struct Level(LogLevel);

#[derive(Debug, Deserialize)]
struct Message(String);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

// Entity
type Entity = u32;

// World
struct World {
    entities: SmallVec<[Entity; 16]>,
    timestamps: HashMap<Entity, Timestamp>,
    levels: HashMap<Entity, Level>,
    messages: HashMap<Entity, Message>,
    next_entity_id: Entity,
}

impl World {
    fn new() -> Self {
        World {
            entities: SmallVec::new(),
            timestamps: HashMap::new(),
            levels: HashMap::new(),
            messages: HashMap::new(),
            next_entity_id: 0,
        }
    }

    fn create_entity(&mut self, timestamp: Timestamp, level: Level, message: Message) -> Entity {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity_id);
        self.timestamps.insert(entity_id, timestamp);
        self.levels.insert(entity_id, level);
        self.messages.insert(entity_id, message);
        entity_id
    }
}

// Systems
async fn log_system(world: &mut World, level: LogLevel, message_text: String) {
    let timestamp = Timestamp(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
    let level = Level(level);
    let message = Message(message_text);
    world.create_entity(timestamp, level, message);
}

async fn search_system<'a>(world: &'a World, query: &str) -> Vec<&'a Message> {
    world
        .messages
        .iter()
        .filter_map(|(&entity, message)| {
            if message.0.contains(query) {
                Some(message)
            } else {
                None
            }
        })
        .collect()
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
        let mut file = OpenOptions::new()
            .create(true)
            .append(true);

        // Create a new log file if it doesn't exist
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("logs.txt") {
            let log_message = buffer.trim();
            if let Err(e) = writeln!(file, "{}", log_message) {
                eprintln!("Failed to write to file: {}", e);
            }
        }

        match OpenOptions::new().create(true).append(true).open("logs.txt") {
            Ok(mut file) => {
                let log_message = buffer.trim();
            }
            Err(e) => {
                eprintln!("Failed to open file: {}", e);
            }
        }
        buffer.clear();
    }
}
