use iced_style::application::{self, Appearance};

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
pub enum Application {
    #[default]
    Default,
}

impl<T> StyleSheet for T
where
    T: DevAuto,
{
    type Style = Application;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.palette();

        match style {
            Application::Default => Appearance {
                background_color: palette.primary.base.base,
                text_color: palette.primary.base.on_base,
            },
        }
    }
}

// ----- Impl the actual trait -----

impl<T> application::StyleSheet for Wrap<T>
where
    T: StyleSheet,
{
    type Style = T::Style;

    #[inline]
    fn appearance(&self, style: &Self::Style) -> Appearance {
        T::appearance(self, style)
    }
}
