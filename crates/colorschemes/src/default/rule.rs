use derive_more::From;
use iced_style::rule::{self, Appearance};

use super::Wrap;
use crate::{ColorScheme, WithColorScheme};

// ----- Mirror trait -----

pub trait StyleSheet {
    type Style: Default;

    fn appearance(&self, style: &Self::Style) -> Appearance;
}

// ----- DevAuto impl -----

pub trait DevAuto: WithColorScheme {}
impl DevAuto for ColorScheme {}

#[derive(Debug, Clone, Default, From)]
pub enum Rule {
    #[default]
    Default,
    Surface,
}

impl<T> StyleSheet for T
where
    T: DevAuto,
{
    type Style = Rule;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.palette();

        match style {
            Rule::Default => Appearance {
                color: palette.primary.strong.base,
                width: 1,
                radius: 0.0,
                fill_mode: rule::FillMode::Percent(90.0),
            },
            Rule::Surface => Appearance {
                color: palette.surface.base.base,
                width: 1,
                radius: 0.0,
                fill_mode: rule::FillMode::Percent(90.0),
            },
        }
    }
}

// ----- Impl the actual trait -----

impl<T> rule::StyleSheet for Wrap<T>
where
    T: StyleSheet,
{
    type Style = T::Style;

    #[inline]
    fn appearance(&self, style: &Self::Style) -> Appearance {
        T::appearance(self, style)
    }
}
