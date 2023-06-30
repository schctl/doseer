//! The catppuccin theme.
//!
//! <https://catppuccin.com>

use crate::{Accented, Brightness, ColorPalette, FullGroup, Group, Pair, WithColorScheme};

mod colors;

/// Catppuccin variants.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Variant {
    #[default]
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl WithColorScheme for Variant {
    #[inline]
    fn brightness(&self) -> Brightness {
        match self {
            Self::Latte => Brightness::Light,
            Self::Frappe | Self::Macchiato | Self::Mocha => Brightness::Dark,
        }
    }

    #[inline]
    fn palette(&self) -> &ColorPalette {
        match self {
            Self::Latte => &LATTE,
            Self::Frappe => &FRAPPE,
            Self::Macchiato => &MACCHIATO,
            Self::Mocha => &MOCHA,
        }
    }
}

macro_rules! def_variant {
    ($name:ident, $path:ident) => {
        const $name: ColorPalette = ColorPalette {
            primary: FullGroup {
                base: Accented {
                    base: colors::$path::BASE,
                    on_base: colors::$path::TEXT,
                    accent: colors::$path::MAUVE,
                    on_accent: colors::$path::CRUST,
                },
                weak: Accented {
                    base: colors::$path::MANTLE,
                    on_base: colors::$path::TEXT,
                    accent: colors::$path::MAUVE,
                    on_accent: colors::$path::CRUST,
                },
                strong: Accented {
                    base: colors::$path::CRUST,
                    on_base: colors::$path::TEXT,
                    accent: colors::$path::MAUVE,
                    on_accent: colors::$path::CRUST,
                },
            },

            // Same base colors
            secondary: FullGroup {
                base: Accented {
                    base: colors::$path::BASE,
                    on_base: colors::$path::SUBTEXT_1,
                    accent: colors::$path::FLAMINGO,
                    on_accent: colors::$path::BASE,
                },
                weak: Accented {
                    base: colors::$path::MANTLE,
                    on_base: colors::$path::SUBTEXT_0,
                    accent: colors::$path::ROSEWATER,
                    on_accent: colors::$path::BASE,
                },
                strong: Accented {
                    base: colors::$path::CRUST,
                    on_base: colors::$path::SUBTEXT_0,
                    accent: colors::$path::PINK,
                    on_accent: colors::$path::BASE,
                },
            },

            surface: Group {
                base: Pair {
                    base: colors::$path::SURFACE_1,
                    on_base: colors::$path::SUBTEXT_1,
                },
                weak: Pair {
                    base: colors::$path::SURFACE_0,
                    on_base: colors::$path::OVERLAY_2,
                },
                strong: Pair {
                    base: colors::$path::SURFACE_2,
                    on_base: colors::$path::SUBTEXT_1,
                },
            },

            overlay: Group {
                base: Pair {
                    base: colors::$path::MANTLE,
                    on_base: colors::$path::TEXT,
                },
                weak: Pair {
                    base: colors::$path::CRUST,
                    on_base: colors::$path::TEXT,
                },
                strong: Pair {
                    base: colors::$path::SURFACE_0,
                    on_base: colors::$path::TEXT,
                },
            },

            success: Group {
                base: Pair {
                    base: colors::$path::GREEN,
                    on_base: colors::$path::BASE,
                },
                weak: Pair {
                    base: colors::$path::LAVENDER,
                    on_base: colors::$path::BASE,
                },
                strong: Pair {
                    base: colors::$path::MAUVE,
                    on_base: colors::$path::BASE,
                },
            },

            error: Group {
                base: Pair {
                    base: colors::$path::MAROON,
                    on_base: colors::$path::BASE,
                },
                weak: Pair {
                    base: colors::$path::PEACH,
                    on_base: colors::$path::BASE,
                },
                strong: Pair {
                    base: colors::$path::RED,
                    on_base: colors::$path::BASE,
                },
            },
        };
    };
}

def_variant!(LATTE, latte);
def_variant!(FRAPPE, frappe);
def_variant!(MACCHIATO, macchiato);
def_variant!(MOCHA, mocha);
