use std::path::Path;

use log::warn;
use notify::{
    Config, 
    RecommendedWatcher, 
    RecursiveMode,
     Watcher
};

pub fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => log_event(event),
            Err(e) => println!("Watch error: {:?}", e),
        }
    }

    Ok(())
}

fn log_event(event: notify::Event) {
    warn!("Change occurred: {:?}", event);
}