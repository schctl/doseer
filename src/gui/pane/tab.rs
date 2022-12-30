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
    pub fn new_with<P: AsRef<Path>>(location: P) -> anyhow::Result<Self> {
        let this = Self {
            location: location.as_ref().to_owned(),
            items: Self::get_items(location.as_ref())?,
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
    ItemUpdate((), usize),
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

        // update anyway
        self.items = Self::get_items(&self.location)?;

        Ok(())
    }

    pub fn view(
        &self,
        opts: ViewOpts,
    ) -> anyhow::Result<iced::Element<'_, Message, iced::Renderer<iced::Theme>>> {
        let mut columns = Column::new();

        let mut iter = self.items.iter();

        // Grid of items
        let num_rows = std::cmp::max(self.items.len() / opts.columns, 1);

        // Column wise
        for _ in 0..num_rows {
            columns = columns.push({
                // Row wise
                let mut row = Row::new().width(iced::Length::Fill);

                for (index, item) in iter.by_ref().enumerate().take(opts.columns) {
                    row = row.push({
                        let view = item.view()?.map(move |m| Message::ItemUpdate(m, index));
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
