//! Some pre-defined colorschemes for the [`iced`] UI library.
//!
//! [`iced`]: https://crates.io/crates/iced

use enum_dispatch::enum_dispatch;
use iced_core::Color;
use std::ops::Deref;

pub mod colorschemes;
#[cfg(feature = "dev-default")]
pub mod default;

#[doc(hidden)]
pub use iced_core;
#[doc(hidden)]
pub use iced_style;
#[doc(hidden)]
pub use paste::paste;

// --- Core definitions ---

/// Describes whether the colorscheme is a dark or light mode theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Brightness {
    Light,
    Dark,
}

impl Brightness {
    #[inline]
    pub const fn is_light(&self) -> bool {
        matches!(self, Self::Light)
    }

    #[inline]
    pub const fn is_dark(&self) -> bool {
        !self.is_light()
    }
}

/// A full set of colors that will (hopefully) allow you to theme anything.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ColorPalette {
    /// Key colors used for most components across the UI.
    pub primary: FullGroup,
    /// Colors for secondary components/less prominent components, like sidebars.
    pub secondary: FullGroup,
    /// Colors for surface elements with different elevations.
    pub surface: Group,
    /// Colors for floating elements.
    pub overlay: Group,
    /// Colors for describing errors/warnings.
    pub success: Group,
    /// Colors for describing successful operations.
    pub error: Group,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pair {
    /// The base color.
    pub base: Color,
    /// A color that's clearly visible over the base.
    pub on_base: Color,
}

/// Like [`Pair`] but with accent colors also.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Accented {
    /// The base color.
    pub base: Color,
    /// A color that's clearly visible over the base.
    pub on_base: Color,
    /// An accent color for `base`.
    pub accent: Color,
    /// A color that's clearly visible over the accent color.
    pub on_accent: Color,
}

/// A set of colors with different "weights".
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Group {
    pub weak: Pair,
    pub base: Pair,
    pub strong: Pair,
}

/// Just like [`Group`] but with accent color groups.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FullGroup {
    pub base: Accented,
    pub weak: Accented,
    pub strong: Accented,
}

// --- Theme definitions ---

/// An interface for generating application wide color palettes.
#[enum_dispatch(ColorScheme)]
pub trait WithColorScheme {
    fn brightness(&self) -> Brightness;
    fn palette(&self) -> &ColorPalette;
}

/// All defined colorschemes.
#[derive(Debug, Clone)]
#[enum_dispatch]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ColorScheme {
    Catppuccin(colorschemes::catppuccin::Variant),
}

/// Auto impl the colorscheme interface for colorscheme holders.
impl<T> WithColorScheme for T
where
    T: Deref<Target = ColorScheme>,
{
    #[inline]
    fn brightness(&self) -> Brightness {
        self.deref().brightness()
    }

    #[inline]
    fn palette(&self) -> &ColorPalette {
        self.deref().palette()
    }
}
