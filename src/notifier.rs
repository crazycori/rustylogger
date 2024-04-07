use notify::{recommended_watcher, RecursiveMode, Result, Event, Config, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use tokio::sync::mpsc::Sender;

pub async fn watch_file_changes(tx: Sender<Event>) -> Result<()> {
    let (std_tx, std_rx) = channel();
    let config = Config::default();
    let mut watcher = recommended_watcher(move |res| {
        if let Ok(event) = res {
            if let Err(e) = std_tx.send(event) {
                eprintln!("Error sending event to std channel: {}", e);
            }
        }
    })?;

    // Watch /var/log
    watcher.watch(Path::new("/var/log/"), RecursiveMode::Recursive)?;

    // Spawn a new Tokio task to forward messages
    tokio::spawn(async move {
        for event in std_rx {
            if let Err(e) = tx.send(event).await {
                eprintln!("Error sending event to Tokio channel: {}", e);
            }
        }
    });

    Ok(())
}
