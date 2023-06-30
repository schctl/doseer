use iced_style::pane_grid::{self, Line};

use super::Wrap;
use crate::{ColorScheme, WithColorScheme};

// ----- Mirror trait -----

pub trait StyleSheet {
    type Style: Default;

    fn picked_split(&self, style: &Self::Style) -> Option<Line>;
    fn hovered_split(&self, style: &Self::Style) -> Option<Line>;
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
}
