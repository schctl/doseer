//! Tab widget.

use std::path::Path;

use m7_core::dirs;
use m7_core::path::PathWrap;
use m7_ui_ext::widgets::grid::{flexbox, uniform};

use iced::widget::{container, scrollable};
use iced::{Command, Length, Size};

use crate::gui::Element;
use crate::item;

pub mod watcher;

/// A single tab displays a single open location.
#[derive(Debug)]
pub struct Tab {
    /// Contents of the current location.
    contents: dirs::Contents,
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
            contents: dirs::Contents::new(path)?,
            selected: None,
        })
    }

    /// Get the location this tab points to.
    #[inline]
    pub fn location(&self) -> &PathWrap {
        self.contents.location()
    }

    /// Change this tab to a new location.
    pub fn update_location<P: AsRef<Path>>(&mut self, new: P) -> anyhow::Result<()> {
        self.contents = dirs::Contents::new(new)?;
        Ok(())
    }
}

// Widget stuff

/// Internal tab message.
#[derive(Debug, Clone)]
pub enum Message {
    Item(item::Message),
    UpdateContents,
    UpdateFail,
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
            self.update_location(path)?;
        } else {
            open::that(path)?;
        }

        Ok(())
    }
}

impl Tab {
    pub fn update(&mut self, message: Message) -> anyhow::Result<Command<Message>> {
        match message {
            Message::Item(m) => {
                match m {
                    item::Message::Click(path) => {
                        if self.is_selected(&path) {
                            self.open(path)?;
                        } else {
                            self.selected = Some(path);
                        }
                    }
                }

                Ok(Command::none())
            }
            Message::UpdateContents => {
                self.contents.update_contents()?;
                Ok(watcher::command(self.location()))
            }

            // Register fail and stop trying
            Message::UpdateFail => {
                tracing::error!("failed to update tab contents");
                Ok(Command::none())
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        flexbox::responsive(|_| {
            let grid = uniform(
                self.contents.contents().iter().map(|path| {
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
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
                }),
                Size {
                    width: 128,
                    height: 128,
                },
            )
            .allow_more_spacing(true);

            scrollable(container(grid).padding(8).width(iced::Length::Fill)).into()
        })
        .into()
    }
}
