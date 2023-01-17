//! A single item (file/folder/whatever) in a directory.

use iced::widget::svg::Handle;
use iced::widget::{button, column, text, Svg};
use iced::Color;
use sleet::style::ColorScheme;

use crate::gui::{icons, theme, Element};
use crate::path::PathWrap;

#[derive(Debug, Clone)]
pub enum Message {
    Click(PathWrap),
    // TODO: Delete, Properties, Copy, Paste, etc
}

pub fn view<'a>(path: PathWrap, theme: Style) -> Element<'a, Message> {
    let icon = Svg::new(Handle::from_memory(icons::DIRECTORY));

    let text = text(path.display().to_string_lossy())
        .horizontal_alignment(iced::alignment::Horizontal::Center);

    button(
        column!(icon, text)
            .align_items(iced::Alignment::Center)
            .width(iced::Length::Fill),
    )
    .width(iced::Length::Fill)
    .on_press(Message::Click(path))
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
        let palette = theme.palette();

        match self {
            Self::Selected => iced::widget::button::Appearance {
                background: Color {
                    a: 0.7,
                    ..palette.primary.base.accent
                }
                .into(),
                text_color: palette.primary.base.on_accent,
                border_radius: 6.0,
                border_color: palette.primary.strong.accent,
                border_width: 1.0,
                ..Default::default()
            },
            Self::Default => iced::widget::button::Appearance {
                background: Color::TRANSPARENT.into(),
                text_color: palette.surface.base.on_base,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }

    pub fn hovered(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
        let palette = theme.palette();

        match self {
            Self::Selected => self.active(theme),
            Self::Default => iced::widget::button::Appearance {
                background: Color {
                    a: 0.8,
                    ..palette.surface.base.base
                }
                .into(),
                text_color: palette.surface.base.on_base,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }
}
