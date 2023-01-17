//! Global theme.

use std::ops::Deref;

use iced::widget::button;
use sleet::style;
use sleet::stylesheet;

use crate::gui::{item, pane};

#[derive(Debug, Clone, Default)]
pub struct Theme(pub style::Theme);

impl Deref for Theme {
    type Target = style::Theme;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl stylesheet::application::Auto for Theme {}
impl stylesheet::container::Auto for Theme {}
impl stylesheet::pane_grid::Auto for Theme {}
impl stylesheet::scrollable::Auto for Theme {}
impl stylesheet::svg::Auto for Theme {}
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
