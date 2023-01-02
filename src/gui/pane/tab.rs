//! Tab widget.

use std::cell::UnsafeCell;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use iced::widget::{container, scrollable};
use iced::widget::{Column, Row};

use super::item;
use crate::dirs;
use crate::gui::Element;

/// A single tab displays a single open location.
#[derive(Debug)]
pub struct Tab {
    /// Update lock.
    update_lock: RwLock<()>,
    /// Contents of the current location.
    contents: UnsafeCell<dirs::Contents>,
}

impl Tab {
    /// Open a new tab with the user's home directory.
    #[inline]
    pub fn new() -> anyhow::Result<Self> {
        Self::new_with(dirs::BASE.home_dir())
    }

    /// Open a new tab with a specified location.
    #[inline]
    pub fn new_with<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Ok(Self {
            update_lock: RwLock::new(()),
            contents: UnsafeCell::new(dirs::Contents::new(path)?),
        })
    }

    /// Get the location this tab points to.
    #[inline]
    pub fn location(&self) -> PathBuf {
        let _read_lock = self.update_lock.read();
        // SAFETY: Read lock held
        (unsafe { &*self.contents.get() }).location().to_owned()
    }
}

// Widget stuff

/// Internal tab message.
#[derive(Debug, Clone)]
pub enum Message {
    ItemUpdate((), usize),
    // TODO: manual update contents
}

pub struct ViewOpts {
    pub columns: usize,
}

impl Tab {
    pub fn update(&mut self, message: Message) -> anyhow::Result<()> {
        match message {
            Message::ItemUpdate(_, _) => {
                // ¯\_(ツ)_/¯
            }
        }

        Ok(())
    }

    pub fn view<'a>(&'a self, opts: ViewOpts) -> anyhow::Result<Element<'a, Message>> {
        let mut columns = Column::new();

        // Update contents
        {
            let _write_lock = self.update_lock.write();
            // SAFETY: Write lock held
            let contents = unsafe { &mut *self.contents.get() };

            let _ = contents.update_contents();
        }

        let _read_lock = self.update_lock.read();
        // SAFETY: Read lock held
        let contents: &'a [PathBuf] = unsafe { &*self.contents.get() }.contents();

        let mut iter = contents.iter();

        // Grid of items
        let num_rows = std::cmp::max(contents.len() / opts.columns, 1);

        // Column wise
        for _ in 0..num_rows {
            columns = columns.push({
                // Row wise
                let mut row = Row::new().width(iced::Length::Fill);

                for (index, path) in iter.by_ref().enumerate().take(opts.columns) {
                    row = row.push({
                        let view = item::view(path)?.map(move |m| Message::ItemUpdate(m, index));

                        container(view).width(iced::Length::Units(128))
                    });
                }

                row
            });
        }

        // Scroll state
        let scrollable = scrollable(columns);

        Ok(scrollable.into())
    }
}
