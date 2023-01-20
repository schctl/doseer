//! File watcher.

use iced::Command;
use m7_core::path::PathWrap;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::runtime::Handle;
use tokio::sync::mpsc::{self, Receiver};

use super::Message;

type EventReceiver = Receiver<notify::Result<Event>>;

/// Create the watcher.
fn create_watcher() -> notify::Result<(RecommendedWatcher, EventReceiver)> {
    let (sender, receiver) = mpsc::channel(16);

    let handle = Handle::current();

    let handler = move |res| {
        handle.block_on(async {
            let _ = sender.send(res).await;
        });
    };

    notify::recommended_watcher(handler).map(|w| (w, receiver))
}

/// Handle or ignore an fs event.
fn handle_notif(result: notify::Result<Event>) -> Option<Message> {
    match result {
        Ok(event) => match event.kind {
            EventKind::Create(_) | EventKind::Remove(_) | EventKind::Modify(_) => {
                Some(Message::UpdateContents)
            }
            _ => None,
        },
        _ => Some(Message::UpdateFail),
    }
}

/// Watch the current location, producing a [`Message`] when an event occurs.
pub async fn watch(path: PathWrap) -> Message {
    if let Ok((mut watcher, mut receiver)) = create_watcher() {
        if watcher.watch(&path, RecursiveMode::NonRecursive).is_err() {
            return Message::UpdateFail;
        }

        loop {
            if let Some(message) = receiver.recv().await.map(handle_notif).flatten() {
                return message;
            }
        }
    }

    Message::UpdateFail
}

pub fn command(path: &PathWrap) -> Command<Message> {
    let path = path.clone();
    Command::perform(watch(path), |x| x)
}
