use iced_core::{BorderRadius, Color};
use iced_style::pane_grid::{self, Appearance, Line};

use super::Wrap;
use crate::{ColorScheme, WithColorScheme};

// ----- Mirror trait -----

pub trait StyleSheet {
    type Style: Default;

    fn picked_split(&self, style: &Self::Style) -> Option<Line>;
    fn hovered_split(&self, style: &Self::Style) -> Option<Line>;
    fn hovered_region(&self, style: &Self::Style) -> Appearance;
}

// ----- DevAuto impl -----

pub trait DevAuto: WithColorScheme {}
impl DevAuto for ColorScheme {}

#[derive(Debug, Clone, Default)]
pub enum PaneGrid {
    #[default]
    Default,
}

impl<T> StyleSheet for T
where
    T: DevAuto,
{
    type Style = PaneGrid;

    fn picked_split(&self, style: &Self::Style) -> Option<Line> {
        let palette = self.palette();

        match style {
            Self::Style::Default => Some(pane_grid::Line {
                color: palette.primary.strong.accent,
                width: 4.0,
            }),
        }
    }

    fn hovered_split(&self, style: &Self::Style) -> Option<Line> {
        let palette = self.palette();

        match style {
            Self::Style::Default => Some(pane_grid::Line {
                color: palette.primary.base.accent,
                width: 4.0,
            }),
        }
    }

    fn hovered_region(&self, style: &Self::Style) -> Appearance {
        let palette = self.palette();

        match style {
            Self::Style::Default => Appearance {
                background: palette.surface.base.base.into(),
                border_width: 0.0,
                border_radius: BorderRadius::from(0.0),
                border_color: Color::TRANSPARENT,
            },
        }
    }
}

// ----- Impl the actual trait -----

impl<T> pane_grid::StyleSheet for Wrap<T>
where
    T: StyleSheet,
{
    type Style = T::Style;

    #[inline]
    fn picked_split(&self, style: &Self::Style) -> Option<Line> {
        T::picked_split(self, style)
    }

    #[inline]
    fn hovered_split(&self, style: &Self::Style) -> Option<Line> {
        T::hovered_split(self, style)
    }

    fn hovered_region(&self, style: &Self::Style) -> Appearance {
        T::hovered_region(&self, style)
    }
}
