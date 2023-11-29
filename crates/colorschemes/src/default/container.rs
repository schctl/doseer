use derive_more::From;
use iced_style::container::{self, Appearance};

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
pub enum Container {
    #[default]
    Transparent,
    Box,
}

impl<T> StyleSheet for T
where
    T: DevAuto,
{
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.palette();

        match style {
            Self::Style::Transparent => Default::default(),
            Self::Style::Box => container::Appearance {
                background: Some(palette.surface.base.base.into()),
                ..Default::default()
            },
        }
    }
}

// ----- Impl the actual trait -----

impl<T> container::StyleSheet for Wrap<T>
where
    T: StyleSheet,
{
    type Style = T::Style;

    #[inline]
    fn appearance(&self, style: &Self::Style) -> Appearance {
        T::appearance(self, style)
    }
}
