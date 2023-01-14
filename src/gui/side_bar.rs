//! Side bar widget.

use std::path::{Path, PathBuf};

use iced::widget::svg::{Handle, Svg};
use iced::widget::{button, column, container, row, text, Column};
use iced::{alignment, Alignment, Length, Padding};

use crate::gui::pane::{self, TabButtonStyle};
use crate::gui::{icons, Element};
use crate::path::PathWrap;

/// The file picker side bar.
#[derive(Debug)]
pub struct SideBar {
    /// Section of the sidebar which contains a set of default paths.
    pub default: Vec<PathBuf>,
    /// Customizable section.
    pub bookmarks: Vec<PathBuf>,
}

fn item_button<'a>(
    path: &Path,
    is_open: impl Fn(&Path) -> bool,
) -> anyhow::Result<Element<'a, pane::area::Message>> {
    let style = // We can reuse this
    if (is_open)(&path) {
        TabButtonStyle::Focused
    } else {
        TabButtonStyle::Default
    }.into();

    let item_button = button(
        row!(
            Svg::new(Handle::from_memory(icons::DIRECTORY))
                .width(Length::Units(22))
                .height(Length::Units(22)),
            text(
                // get file name location
                path.canonicalize()?
                    .file_name()
                    // unwrap ok since name is canonicalized
                    .unwrap()
                    .to_string_lossy(),
            )
            .size(22),
        )
        .spacing(6)
        .align_items(Alignment::Center)
        .width(Length::Units(186))
        .height(Length::Units(28))
        .padding(Padding::from([0, 4])),
    )
    // focus tab when the button is pressed
    .on_press(pane::area::Message::Pane(
        pane::Message::Tab(pane::TabMessage::Replace(PathWrap::from_path(path)?), None),
        None,
    ))
    .style(style);

    Ok(item_button.into())
}

fn separator<'a>() -> Element<'a, pane::area::Message> {
    container(text(""))
        .width(Length::Units(1))
        .height(Length::Units(1))
        .into()
}

impl SideBar {
    pub fn view(
        &self,
        is_open: impl Fn(&Path) -> bool,
    ) -> anyhow::Result<Element<pane::area::Message>> {
        let title = container(text("Files").size(28))
            .height(pane::Pane::TOP_BAR_HEIGHT)
            .align_y(alignment::Vertical::Center)
            .padding([0, 8]);

        let mut col = Column::new();

        for path in &self.default {
            col = col.push(item_button(path, &is_open)?);
        }

        col = col.push(separator());

        for path in &self.bookmarks {
            col = col.push(item_button(path, &is_open)?);
        }

        Ok(column!(title, col).into())
    }
}
