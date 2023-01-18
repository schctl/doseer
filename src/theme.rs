//! Global theme.

use derive_more::{Deref, From};
use iced::widget::button;
use sleet::{style, stylesheet};

use crate::{item, pane};

#[derive(Debug, Clone, Default, From, Deref)]
pub struct Theme(pub style::Theme);

impl stylesheet::application::Auto for Theme {}
impl stylesheet::container::Auto for Theme {}
impl stylesheet::pane_grid::Auto for Theme {}
impl stylesheet::scrollable::Auto for Theme {}
impl stylesheet::text::Auto for Theme {}

#[derive(Debug, Clone, Default)]
pub enum Button {
    #[default]
    Default,
    Tab(pane::TabButtonStyle),
    Item(item::Style),
}

impl stylesheet::button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Self::Style::Tab(t) => t.active(self),
            Self::Style::Item(t) => t.active(self),
            _ => <style::Theme as stylesheet::button::StyleSheet>::active(
                &self.0,
                &Default::default(),
            ),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Self::Style::Tab(t) => t.hovered(self),
            Self::Style::Item(t) => t.hovered(self),
            _ => <style::Theme as stylesheet::button::StyleSheet>::hovered(
                &self.0,
                &Default::default(),
            ),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Self::Style::Tab(t) => t.pressed(self),
            Self::Style::Item(t) => t.pressed(self),
            _ => <style::Theme as stylesheet::button::StyleSheet>::pressed(
                &self.0,
                &Default::default(),
            ),
        }
    }
}

pub mod svg {
    use sleet::style::ColorScheme;

    use super::*;

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Neutral {
        /// Lower brightness
        Bright0,
        #[default]
        /// Higher brightness
        Bright1,
    }

    #[derive(Debug, Clone, Copy, Default, From)]
    pub enum Svg {
        #[default]
        Default,
        /// An svg with neutral tones
        Neutral(Neutral),
    }

    impl stylesheet::svg::StyleSheet for Theme {
        type Style = Svg;

        fn appearance(&self, style: &Self::Style) -> iced::widget::svg::Appearance {
            let palette = self.palette();

            iced::widget::svg::Appearance {
                color: match style {
                    Svg::Default => None,
                    Svg::Neutral(tone) => match tone {
                        Neutral::Bright0 => palette.surface.weak.on_base.into(),
                        Neutral::Bright1 => palette.surface.base.on_base.into(),
                    },
                },
            }
        }
    }
}
