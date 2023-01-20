//! Side bar widget.

use std::path::Path;

use m7_core::path::PathWrap;

use iced::widget::{button, column, container, row, text, Column};
use iced::{alignment, Alignment, Color, Length, Padding};
use sleet::style::ColorScheme;

use crate::gui::Element;
use crate::{pane, theme, Icon};

/// The file picker side bar.
#[derive(Debug)]
pub struct SideBar {
    /// Section of the sidebar which contains a set of default paths.
    pub default: Vec<PathWrap>,
    /// Customizable section.
    pub bookmarks: Vec<PathWrap>,
}

/// Tab button theme.
#[derive(Debug, Clone, Default)]
pub enum ButtonStyle {
    #[default]
    Default,
    Focused,
}

impl From<ButtonStyle> for theme::button::Button {
    fn from(t: ButtonStyle) -> Self {
        Self::SideBar(t)
    }
}

impl ButtonStyle {
    pub fn active(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
        let palette = theme.palette();

        match self {
            Self::Focused => iced::widget::button::Appearance {
                background: palette.surface.base.base.into(),
                text_color: palette.surface.base.on_base,
                border_radius: 6.0,
                ..Default::default()
            },
            Self::Default => iced::widget::button::Appearance {
                background: Color::TRANSPARENT.into(),
                text_color: palette.primary.base.on_base,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }

    pub fn hovered(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
        let palette = theme.palette();

        match self {
            Self::Focused => self.active(theme),
            Self::Default => iced::widget::button::Appearance {
                background: palette.surface.weak.base.into(),
                text_color: palette.surface.weak.on_base,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }

    pub fn pressed(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
        Self::Focused.active(theme)
    }
}

fn item_button(
    path: &PathWrap,
    is_open: impl Fn(&Path) -> bool,
) -> anyhow::Result<Element<pane::area::Message>> {
    let style = // We can reuse this
    if (is_open)(path) {
        ButtonStyle::Focused
    } else {
        ButtonStyle::Default
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
        .width(Length::Fill)
        .height(Length::Fill),
    )
    // focus tab when the button is pressed
    .on_press(pane::area::Message::Pane(
        pane::Message::Replace(PathWrap::from_path(path)?),
        None,
    ))
    .width(Length::Fill)
    .height(Length::Units(38))
    .padding(Padding::from([4, 8]))
    .style(style);

    Ok(item_button.into())
}

// TODO: probably should replace this with `Canvas`.
fn separator<'a>() -> Element<'a, pane::area::Message> {
    container(
        container(text(""))
            .style(theme::container::Container::OnBase)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .padding(4)
    .width(Length::Fill)
    .height(Length::Units(4 * 2 + 1))
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

        let mut col = Column::new()
            .align_items(Alignment::Center)
            .padding(8)
            .spacing(4);

        for path in &self.default {
            col = col.push(item_button(path, &is_open)?);
        }

        col = col.push(separator());

        for path in &self.bookmarks {
            col = col.push(item_button(path, &is_open)?);
        }

        Ok(container(column!(title, col))
            .style(theme::container::Container::Weak)
            .width(Length::Fill)
            .height(Length::Fill)
            .into())
    }
}
