//! Global theme.

use derive_more::{Deref, From};
use doseer_colorschemes::default;
use doseer_colorschemes::{ColorScheme, WithColorScheme};

use crate::{content, item, side_bar};

#[derive(Debug, Clone, From, Deref)]
pub struct Theme(ColorScheme);

impl Default for Theme {
    fn default() -> Self {
        Self(ColorScheme::Catppuccin(
            doseer_colorschemes::colorschemes::catppuccin::Variant::Mocha,
        ))
    }
}

impl WithColorScheme for Theme {
    fn palette(&self) -> &doseer_colorschemes::ColorPalette {
        self.0.palette()
    }

    fn brightness(&self) -> doseer_colorschemes::Brightness {
        self.0.brightness()
    }
}

impl default::application::DevAuto for Theme {}
impl default::pane_grid::DevAuto for Theme {}
impl default::rule::DevAuto for Theme {}
impl default::scrollable::DevAuto for Theme {}

pub mod button {
    use super::*;

    use iced::widget::button;

    #[derive(Debug, Clone, Default)]
    pub enum Button {
        #[default]
        Default,
        Tab(content::TabButtonStyle),
        SideBar(side_bar::ButtonStyle),
        Item(item::Style),
    }

    impl default::button::StyleSheet for Theme {
        type Style = Button;

        fn active(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Self::Style::Tab(t) => t.active(self),
                Self::Style::SideBar(t) => t.active(self),
                Self::Style::Item(t) => t.active(self),
                _ => <ColorScheme as default::button::StyleSheet>::active(
                    &self.0,
                    &Default::default(),
                ),
            }
        }

        fn hovered(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Self::Style::Tab(t) => t.hovered(self),
                Self::Style::SideBar(t) => t.hovered(self),
                Self::Style::Item(t) => t.hovered(self),
                _ => <ColorScheme as default::button::StyleSheet>::hovered(
                    &self.0,
                    &Default::default(),
                ),
            }
        }

        fn pressed(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Self::Style::Tab(t) => t.pressed(self),
                Self::Style::SideBar(t) => t.pressed(self),
                Self::Style::Item(t) => t.pressed(self),
                _ => <ColorScheme as default::button::StyleSheet>::pressed(
                    &self.0,
                    &Default::default(),
                ),
            }
        }
    }
}

pub mod container {
    use super::*;

    #[derive(Debug, Clone, Default)]
    pub enum Container {
        /// A transparent box.
        #[default]
        Default,
        /// A box with a less emphasized color.
        Weak,
    }

    impl default::container::StyleSheet for Theme {
        type Style = Container;

        fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
            let palette = self.palette();

            match style {
                Container::Default => Default::default(),
                Container::Weak => iced::widget::container::Appearance {
                    background: Some(palette.primary.weak.base.into()),
                    ..Default::default()
                },
            }
        }
    }
}

pub mod svg {
    use super::*;

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Neutral {
        /// Lower brightness
        Bright0,
        #[default]
        /// Higher brightness
        Bright1,
    }

    #[derive(Debug, Clone, Copy, Default, From)]
    pub enum Svg {
        #[default]
        Default,
        /// An svg with neutral tones
        Neutral(Neutral),
    }

    impl default::svg::StyleSheet for Theme {
        type Style = Svg;

        fn appearance(&self, style: &Self::Style) -> iced::widget::svg::Appearance {
            let palette = self.palette();

            iced::widget::svg::Appearance {
                color: match style {
                    Svg::Default => None,
                    Svg::Neutral(tone) => match tone {
                        Neutral::Bright0 => palette.surface.weak.on_base.into(),
                        Neutral::Bright1 => palette.surface.base.on_base.into(),
                    },
                },
            }
        }
    }
}

pub mod text {
    use super::*;

    #[derive(Debug, Clone, Copy, Default)]
    pub enum Text {
        #[default]
        Default,
    }

    impl default::text::StyleSheet for Theme {
        type Style = Text;

        fn appearance(&self, style: Self::Style) -> iced::widget::text::Appearance {
            let palette = self.palette();

            match style {
                Text::Default => iced::widget::text::Appearance {
                    color: palette.primary.base.on_base.into(),
                },
            }
        }
    }
}

pub mod fonts {
    use iced::{font::*, Command};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub enum Roboto {
        #[default]
        Regular,
    }

    impl From<Roboto> for Font {
        fn from(value: Roboto) -> Self {
            match value {
                Roboto::Regular => Font::with_name("Roboto"),
            }
        }
    }

    impl Roboto {
        pub const BYTES: &'static [u8] =
            include_bytes!("../assets/static/fonts/Roboto/Roboto-Regular.ttf");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub enum RobotoMono {
        #[default]
        Regular,
        Bold,
    }

    impl From<RobotoMono> for Font {
        fn from(value: RobotoMono) -> Self {
            match value {
                RobotoMono::Regular => Font::with_name("Roboto"),
                RobotoMono::Bold => Font {
                    weight: Weight::Bold,
                    ..Font::with_name("Roboto")
                },
            }
        }
    }

    impl RobotoMono {
        pub const BYTES: &'static [u8] =
            include_bytes!("../assets/static/fonts/RobotoMono/RobotoMono-VariableFont_wght.ttf");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub enum SofiaSans {
        #[default]
        Regular,
        Black,
    }

    impl From<SofiaSans> for Font {
        fn from(value: SofiaSans) -> Self {
            match value {
                SofiaSans::Regular => Font::with_name("Sofia Sans"),
                SofiaSans::Black => Font {
                    weight: Weight::Black,
                    ..Font::with_name("Sofia Sans")
                },
            }
        }
    }

    impl SofiaSans {
        pub const BYTES: &'static [u8] =
            include_bytes!("../assets/static/fonts/Sofia_Sans/SofiaSans-VariableFont_wght.ttf");
    }

    /// Load all required fonts.
    pub fn load_all() -> Command<Result<(), Error>> {
        Command::batch([Roboto::BYTES, RobotoMono::BYTES, SofiaSans::BYTES].map(load))
    }
}
