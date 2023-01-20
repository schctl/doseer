//! File watcher.

use std::path::Path;
use std::sync::{Arc, Mutex};

use iced::Command;
use m7_core::path::PathWrap;
use notify::{Event, EventKind, RecursiveMode, Watcher};

use super::Message;

/// Watch the current location, blocking the current thread until a handled event occurs.
pub fn watch(path: &Path) -> Message {
    let message = Arc::new(Mutex::new(None));

    // Create our event handler
    let register = message.clone();
    let handler = move |res: notify::Result<Event>| {
        let mut lock = register.lock().unwrap();

        // Watch for the events we need.
        // This is where we produce the tab messages.
        match res {
            Ok(event) => {
                match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                        *lock = Some(Message::UpdateContents);
                    }
                    _ => {
                        // Ignore this event
                    }
                }
            }
            _ => *lock = Some(Message::UpdateFail),
        }
    };

    // Block and watch
    if let Ok(mut watcher) = notify::recommended_watcher(handler) {
        loop {
            let ret = watcher.watch(path, RecursiveMode::NonRecursive);

            match ret {
                Ok(_) => {
                    let lock = message.lock().unwrap();

                    if let Some(m) = &*lock {
                        return m.clone();
                    }

                    // ...
                    // Keep watching until we get a message from a handled event
                }
                // Watch failed: abort
                _ => return Message::UpdateFail,
            }
        }
    } else {
        // Could not create watcher: abort
        Message::UpdateFail
    }
}

pub fn command(path: &PathWrap) -> Command<Message> {
    let path = path.clone();
    let task = tokio::task::spawn_blocking(move || watch(&path));
    Command::perform(task, |res| res.unwrap_or(Message::UpdateFail))
}
