//! Global theme.

use iced::widget::button;
use sleet::style::{sample, stylesheet, theme};

use crate::gui::{item, pane};

theme::theme! {
    Theme
    into_other(sample::Theme)
}

stylesheet::impl_all! {
    [Application, Container, PaneGrid, Svg, Scrollable, Text] for Theme;
}

#[derive(Debug, Clone, Default)]
pub enum Button {
    #[default]
    Default,
    Tab(pane::TabButtonStyle),
    Item(item::Style),
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Self::Style::Tab(t) => t.active(self),
            Self::Style::Item(t) => t.active(self),
            _ => sample::default_style!(Button::active(self)),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Self::Style::Tab(t) => t.hovered(self),
            Self::Style::Item(t) => t.hovered(self),
            _ => sample::default_style!(Button::hovered(self)),
        }
    }
}
