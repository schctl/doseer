//! Tab widget.

use std::cell::UnsafeCell;
use std::path::Path;
use std::sync::RwLock;

use m7_core::dirs;
use m7_core::path::PathWrap;
use m7_ui_ext::widgets::grid::flexbox;

use iced::widget::{container, scrollable};

use crate::gui::Element;
use crate::item;

/// A single tab displays a single open location.
#[derive(Debug)]
pub struct Tab {
    /// Update lock.
    update_lock: RwLock<()>,
    /// Contents of the current location.
    contents: UnsafeCell<dirs::Contents>,
    /// The currently selected item.
    selected: Option<PathWrap>,
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
            selected: None,
        })
    }

    /// Get the location this tab points to.
    #[inline]
    pub fn location(&self) -> &PathWrap {
        let _read_lock = self.update_lock.read();
        // SAFETY: Read lock held
        (unsafe { &*self.contents.get() }).location()
    }

    /// Change this tab to a new location.
    pub fn update_location<P: AsRef<Path>>(&mut self, new: P) -> anyhow::Result<()> {
        self.contents = UnsafeCell::new(dirs::Contents::new(new)?);
        Ok(())
    }
}

// Widget stuff

/// Internal tab message.
#[derive(Debug, Clone)]
pub enum Message {
    Item(item::Message),
    // TODO: manual update contents
}

impl Tab {
    fn is_selected<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(selected) = &self.selected {
            if selected.as_ref() == path.as_ref() {
                return true;
            }
        }

        false
    }

    /// Open a path.
    fn open<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<()> {
        let path = path.as_ref();

        if path.is_dir() {
            self.update_location(path)?
        } else {
            open::that(path)?;
        }

        Ok(())
    }
}

impl Tab {
    pub fn update(&mut self, message: Message) -> anyhow::Result<()> {
        match message {
            Message::Item(m) => match m {
                item::Message::Click(path) => {
                    if self.is_selected(&path) {
                        self.open(path)?;
                    } else {
                        self.selected = Some(path);
                    }
                }
            },
        }

        Ok(())
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        // Update contents
        {
            let _write_lock = self.update_lock.write();
            // SAFETY: Write lock held
            let contents = unsafe { &mut *self.contents.get() };

            let _ = contents.update_contents();
        }

        let _read_lock = self.update_lock.read();
        // SAFETY: Read lock held
        let contents: &'a [PathWrap] = unsafe { &*self.contents.get() }.contents();

        flexbox::responsive(|_| {
            let grid = flexbox(contents.iter().map(|path| {
                container(
                    item::view(
                        path.clone(),
                        if self.is_selected(path) {
                            item::Style::Selected
                        } else {
                            item::Style::Default
                        },
                    )
                    .map(Message::Item),
                )
                .width(iced::Length::Units(128))
                .height(iced::Length::Units(128))
                .into()
            }));

            scrollable(container(grid).padding(8).width(iced::Length::Fill)).into()
        })
        .into()
    }
}
