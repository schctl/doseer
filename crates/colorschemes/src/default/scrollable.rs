use iced_core::{BorderRadius, Color};
use iced_style::scrollable::{self, Scrollbar};

use super::Wrap;
use crate::{ColorScheme, WithColorScheme};

// ----- Mirror trait -----

pub trait StyleSheet {
    type Style: Default;

    fn active(&self, style: &Self::Style) -> Scrollbar;
    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar;

    fn dragging(&self, style: &Self::Style) -> Scrollbar {
        self.hovered(style, false)
    }
}

// ----- DevAuto impl -----

pub trait DevAuto: WithColorScheme {}
impl DevAuto for ColorScheme {}

#[derive(Debug, Clone, Default)]
pub enum Scrollable {
    #[default]
    Default,
}

impl<T> StyleSheet for T
where
    T: DevAuto,
{
    type Style = Scrollable;

    fn active(&self, style: &Self::Style) -> Scrollbar {
        match style {
            Self::Style::Default => Scrollbar {
                background: None,
                border_radius: BorderRadius::from(0.0),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: Color::TRANSPARENT,
                    border_radius: BorderRadius::from(0.0),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_mouse_over_scrollbar: bool) -> Scrollbar {
        let palette = self.palette();

        match style {
            Self::Style::Default => Scrollbar {
                background: None,
                border_radius: BorderRadius::from(0.0),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: palette.primary.base.on_base,
                    border_radius: BorderRadius::from(0.0),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            },
        }
    }
}

// ----- Impl the actual trait -----

impl<T> scrollable::StyleSheet for Wrap<T>
where
    T: StyleSheet,
{
    type Style = T::Style;

    #[inline]
    fn active(&self, style: &Self::Style) -> Scrollbar {
        T::active(self, style)
    }

    #[inline]
    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        T::hovered(self, style, is_mouse_over_scrollbar)
    }

    #[inline]
    fn dragging(&self, style: &Self::Style) -> Scrollbar {
        T::dragging(self, style)
    }
}
