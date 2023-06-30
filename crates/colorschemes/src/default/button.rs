use iced_core::{Background, Color, Vector};
use iced_style::button::{self, Appearance};

use super::Wrap;
use crate::{ColorScheme, WithColorScheme};

// ----- Mirror trait -----

pub trait StyleSheet {
    type Style: Default;

    fn active(&self, style: &Self::Style) -> Appearance;
    fn hovered(&self, style: &Self::Style) -> Appearance;

    fn pressed(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Vector::default(),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        let active = self.active(style);

        Appearance {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

// ----- DevAuto impl -----

pub trait DevAuto: WithColorScheme {}
impl DevAuto for ColorScheme {}

#[derive(Debug, Clone, Default)]
pub enum Button {
    #[default]
    Default,
}

impl<T> StyleSheet for T
where
    T: DevAuto,
{
    type Style = Button;

    fn active(&self, style: &Self::Style) -> Appearance {
        let palette = self.palette();

        match style {
            Button::Default => Appearance {
                background: palette.primary.base.accent.into(),
                text_color: palette.primary.base.on_accent,
                border_radius: 2.0,
                ..Appearance::default()
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        let active = self.active(style);

        let background = match active.background {
            Some(Background::Color(c)) => {
                if self.brightness().is_light() {
                    Color {
                        r: c.r - 0.07,
                        g: c.g - 0.07,
                        b: c.b - 0.07,
                        a: c.a,
                    }
                    .into()
                } else {
                    Color {
                        r: c.r + 0.07,
                        g: c.g + 0.07,
                        b: c.b + 0.07,
                        a: c.a,
                    }
                    .into()
                }
            }
            _ => active.background,
        };

        match style {
            Button::Default => Appearance {
                background,
                ..self.active(style)
            },
        }
    }
}

// ----- Impl the actual trait -----

impl<T> button::StyleSheet for Wrap<T>
where
    T: StyleSheet,
{
    type Style = T::Style;

    #[inline]
    fn active(&self, style: &Self::Style) -> Appearance {
        T::active(self, style)
    }

    #[inline]
    fn hovered(&self, style: &Self::Style) -> Appearance {
        T::hovered(self, style)
    }

    #[inline]
    fn pressed(&self, style: &Self::Style) -> Appearance {
        T::pressed(self, style)
    }

    #[inline]
    fn disabled(&self, style: &Self::Style) -> Appearance {
        T::disabled(self, style)
    }
}
