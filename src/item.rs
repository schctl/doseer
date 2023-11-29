//! A single item (file/folder/whatever) in a directory.

use std::path::Path;

use doseer_colorschemes::WithColorScheme;
use doseer_core::path::PathWrap;
use doseer_icon_loader::file::{ImageOrSvg, Loader};

use iced::widget::{button, column, container, image, svg, text};
use iced::{alignment, Background, BorderRadius, Color, Length, Size};
use lazy_static::lazy_static;

use crate::gui::Element;
use crate::{theme, Icon};

lazy_static! {
    static ref ICONS: Loader = Loader::new();
}

/// Dimensions of an item button.
pub const DIMENSIONS: Size = Size {
    width: 128.0,
    height: 140.0,
};

pub const ICON_DIMENSIONS: Size = Size {
    width: 96.0,
    height: 100.0,
};

#[derive(Debug, Clone)]
pub enum Message {
    Select(PathWrap),
    Deselect,
    // TODO: Delete, Properties, Copy, Paste, etc
}

pub fn icon<'a, P: AsRef<Path>>(path: P) -> Element<'a, Message> {
    let icon = ICONS.load(path.as_ref());

    match icon {
        Some(i) => match i.as_ref().clone() {
            ImageOrSvg::Image(im) => image(im).width(Length::Fill).height(Length::Fill).into(),
            ImageOrSvg::Svg(s) => svg(s).width(Length::Fill).height(Length::Fill).into(),
        },
        None => Icon::Directory.svg().into(),
    }
}

pub fn view<'a>(path: PathWrap, theme: Style) -> Element<'a, Message> {
    let icon = container(
        container(icon(&path))
            .width(Length::Fixed(ICON_DIMENSIONS.width))
            .height(Length::Fixed(ICON_DIMENSIONS.height)),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let text = text(path.display().to_string_lossy())
        .size(20)
        .font(theme::fonts::Roboto::Regular)
        .horizontal_alignment(iced::alignment::Horizontal::Center);

    button(
        column!(icon, text)
            .align_items(iced::Alignment::Center)
            .width(iced::Length::Fill),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .on_press(Message::Select(path))
    .style(theme.into())
    .into()
}

#[derive(Debug, Clone, Default)]
pub enum Style {
    #[default]
    Default,
    Selected,
}

impl From<Style> for theme::button::Button {
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
                background: Some(
                    Color {
                        a: 0.7,
                        ..palette.primary.base.accent
                    }
                    .into(),
                ),
                text_color: palette.primary.base.on_accent,
                border_radius: BorderRadius::from(6.0),
                border_color: palette.primary.strong.accent,
                border_width: 1.0,
                ..Default::default()
            },
            Self::Default => iced::widget::button::Appearance {
                background: None,
                text_color: palette.surface.base.on_base,
                border_radius: BorderRadius::from(6.0),
                ..Default::default()
            },
        }
    }

    pub fn hovered(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
        let palette = theme.palette();

        match self {
            Self::Selected => self.active(theme),
            Self::Default => iced::widget::button::Appearance {
                background: Some(
                    Color {
                        a: 0.8,
                        ..palette.surface.base.base
                    }
                    .into(),
                ),
                text_color: palette.surface.base.on_base,
                border_radius: BorderRadius::from(6.0),
                ..Default::default()
            },
        }
    }

    pub fn pressed(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
        let hovered = self.hovered(theme);

        let background = match hovered.background {
            Some(Background::Color(c)) => Some({
                if theme.brightness().is_light() {
                    Color {
                        r: c.r - 0.07,
                        g: c.g - 0.07,
                        b: c.b - 0.07,
                        a: c.a,
                    }
                    .into()
                } else {
                    Color {
                        r: c.r + 0.07,
                        g: c.g + 0.07,
                        b: c.b + 0.07,
                        a: c.a,
                    }
                    .into()
                }
            }),
            _ => hovered.background,
        };

        iced::widget::button::Appearance {
            background,
            ..hovered
        }
    }
}
