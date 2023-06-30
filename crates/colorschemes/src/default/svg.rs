use iced_style::svg::{self, Appearance};

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

#[derive(Debug, Clone, Default)]
pub enum Svg {
    #[default]
    Default,
}

impl<T> StyleSheet for T
where
    T: DevAuto,
{
    type Style = Svg;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Default::default()
    }
}

// ----- Impl the actual trait -----

impl<T> svg::StyleSheet for Wrap<T>
where
    T: StyleSheet,
{
    type Style = T::Style;

    #[inline]
    fn appearance(&self, style: &Self::Style) -> Appearance {
        T::appearance(self, style)
    }
}
