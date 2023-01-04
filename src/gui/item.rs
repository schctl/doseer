//! A single item (file/folder/whatever) in a directory.

use std::path::{Path, PathBuf};

use iced::widget::svg::Handle;
use iced::widget::{button, column, Svg, Text};
use iced::Color;

use crate::gui::{icons, theme, Element};

#[derive(Debug, Clone)]
pub enum Message {
    Click(PathBuf),
    // TODO: Delete, Properties, Copy, Paste, etc
}

pub fn view(path: &Path, theme: Style) -> Element<Message> {
    let icon = Svg::new(Handle::from_memory(icons::DIRECTORY));

    let text = Text::new(path.as_os_str().to_string_lossy())
        .horizontal_alignment(iced::alignment::Horizontal::Center);

    button(
        column!(icon, text)
            .align_items(iced::Alignment::Center)
            .width(iced::Length::Fill),
    )
    .width(iced::Length::Fill)
    .on_press(Message::Click(path.to_owned()))
    .style(theme.into())
    .into()
}

#[derive(Debug, Clone, Default)]
pub enum Style {
    #[default]
    Default,
    Selected,
}

impl From<Style> for theme::Button {
    #[inline]
    fn from(t: Style) -> Self {
        Self::Item(t)
    }
}

impl Style {
    pub fn active(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
        let base = theme.base();
        let normal = theme.normal();

        match self {
            Self::Selected => iced::widget::button::Appearance {
                background: normal.bg.into(),
                text_color: base.fg,
                border_radius: 6.0,
                ..Default::default()
            },
            Self::Default => iced::widget::button::Appearance {
                background: Color::TRANSPARENT.into(),
                text_color: base.fg,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }

    pub fn hovered(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
        let base = theme.base();
        let normal = theme.normal();

        match self {
            Self::Selected => self.active(theme),
            Self::Default => iced::widget::button::Appearance {
                background: normal.bg.into(),
                text_color: base.fg,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }
}
