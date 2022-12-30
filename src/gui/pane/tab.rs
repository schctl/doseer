//! Tab widget.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

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
    items: Vec<PathBuf>,
    /// Last modified time of this location.
    ///
    /// Used to auto-update the contents.
    modified: SystemTime,
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
        let location_ref = location.as_ref();

        let items = Self::get_items(location_ref)?;
        let location = location_ref.to_owned();

        let modified = match location.metadata()?.modified() {
            Ok(time) => time,
            Err(_) => SystemTime::now(),
        };

        Ok(Self {
            location,
            items,
            modified,
        })
    }

    /// Get the location this tab points to.
    #[inline]
    pub fn location(&self) -> &Path {
        &self.location
    }

    /// Get items in this location.
    fn get_items(path: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let dir = path
            .read_dir()?
            .filter_map(|e| match e {
                Ok(entry) => Some(entry.path().to_owned()),
                _ => None,
            })
            // TODO: collect_into when its stabilized
            .collect::<Vec<_>>();

        Ok(dir)
    }

    /// Update contents if needed.
    fn update_contents(&mut self) -> anyhow::Result<()> {
        if let Ok(metadata) = self.location.metadata() {
            if let Ok(modified) = metadata.modified() {
                if modified > self.modified {
                    self.items = Self::get_items(&self.location)?;
                }
            }
        }

        Ok(())
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

        // Try to update as frequently as we can.
        // Ignore failures, since users should be able to manually refresh
        // and we can deal with errors then.
        let _ = self.update_contents();

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
                        let view = Item::new(item).view()?.map(move |m| Message::ItemUpdate(m, index));
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
