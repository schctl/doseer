//! Side bar widget.

use std::path::Path;

use m7_core::path::PathWrap;

use iced::widget::{button, column, container, row, text, Column};
use iced::{alignment, Alignment, Length, Padding};

use crate::gui::Element;
use crate::pane::{self, TabButtonStyle};
use crate::Icon;

/// The file picker side bar.
#[derive(Debug)]
pub struct SideBar {
    /// Section of the sidebar which contains a set of default paths.
    pub default: Vec<PathWrap>,
    /// Customizable section.
    pub bookmarks: Vec<PathWrap>,
}

fn item_button<'a>(
    path: &PathWrap,
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
            Icon::Directory
                .svg()
                .width(Length::Units(22))
                .height(Length::Units(22)),
            text(path.display().to_string_lossy()).size(22),
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
