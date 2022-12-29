//! Tab widget.

use std::path::{Path, PathBuf};

use iced::widget::{container, scrollable};
use iced::widget::{svg::Handle, Column, Row, Svg, Text};
use iced::Command;

use crate::dirs;

use super::item::Item;

/// Preserved scroll state of this tab.
#[derive(Debug, Default)]
struct ScrollableState {
    offset: f32,
}

/// A single tab displays a single open location.
#[derive(Debug)]
pub struct Tab {
    /// The currently open location.
    location: PathBuf,
    /// Items in current location.
    items: Vec<Item>,
    /// Preserved scroll state.
    scroll: ScrollableState,
}

impl Tab {
    /// Open a new tab with the user's home directory.
    #[inline]
    pub fn new() -> anyhow::Result<Self> {
        Self::new_with(dirs::BASE.home_dir())
    }

    /// Open a new tab with a specified location.
    #[inline]
    pub fn new_with(location: &Path) -> anyhow::Result<Self> {
        let this = Self {
            location: location.to_owned(),
            items: Self::get_items(location)?,
            scroll: ScrollableState::default(),
        };

        Ok(this)
    }

    /// Get the location this tab points to.
    #[inline]
    pub fn location(&self) -> &Path {
        &self.location
    }

    /// Get items in this location.
    fn get_items(path: &Path) -> anyhow::Result<Vec<Item>> {
        let dir = path
            .read_dir()?
            .filter_map(|e| match e {
                Ok(entry) => Some(Item::new(entry.path())),
                _ => None,
            })
            // TODO: collect_into when its stabilized
            .collect::<Vec<_>>();

        Ok(dir)
    }
}

// Widget stuff

/// Internal tab message.
#[derive(Debug, Clone)]
pub enum Message {
    Scrolled(f32),
    Update,
    UpdateLocation(PathBuf),
    ItemUpdate(()),
}

pub struct ViewOpts {
    pub columns: usize,
}

impl Tab {
    pub fn update(&mut self, message: Message) -> anyhow::Result<()> {
        match message {
            Message::Update => {
                self.items = Self::get_items(&self.location)?;
            }
            Message::UpdateLocation(loc) => {
                self.location = loc;
                self.items = Self::get_items(&self.location)?;
            }
            _ => {}
        };

        Ok(())
    }

    pub fn view(
        &self,
        opts: ViewOpts,
    ) -> anyhow::Result<iced::Element<'_, Message, iced::Renderer<iced::Theme>>> {
        let mut columns = Column::new();

        let mut iter = self.items.iter();

        // Grid of items
        for _ in 0..(self.items.len() / opts.columns) {
            let mut row = Row::new().width(iced::Length::Fill);

            for item in iter.by_ref().take(opts.columns) {
                row = row.push(
                    container(item.view()?.map(Message::ItemUpdate))
                        .width(iced::Length::Units(128)),
                );
            }

            columns = columns.push(row);
        }

        // Scroll state
        let scrollable = scrollable(columns);

        Ok(scrollable.into())
    }
}
