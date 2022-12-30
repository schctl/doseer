//! A single item (file/folder/whatever) in a directory.

use std::path::{Path, PathBuf};

use iced::widget::svg::Handle;
use iced::widget::{column, container, Svg, Text};

use super::icons;

/// A single item.
#[derive(Debug, Clone)]
pub struct Item {
    /// Path to this item.
    path: PathBuf,
}

impl<T> From<T> for Item
where
    T: ToOwned<Owned = PathBuf>,
{
    #[inline]
    fn from(path: T) -> Self {
        Self::new(path)
    }
}

impl Item {
    #[inline]
    pub fn new<T>(path: T) -> Self
    where
        T: ToOwned<Owned = PathBuf>,
    {
        Self {
            path: path.to_owned(),
        }
    }

    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Item {
    pub fn view(&self) -> anyhow::Result<iced::Element<'_, (), iced::Renderer<iced::Theme>>> {
        let icon = Svg::new(Handle::from_memory(icons::DIRECTORY));

        let text = Text::new(self.path.as_os_str().to_string_lossy())
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        Ok(container(
            column!(icon, text)
                .align_items(iced::Alignment::Center)
                .width(iced::Length::Fill),
        )
        .width(iced::Length::Fill)
        .into())
    }
}
