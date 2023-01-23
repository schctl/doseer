//! Global theme.

use derive_more::{Deref, From};
use sleet::stylesheet;
use sleet::ColorScheme;

use crate::{item, pane, side_bar};

#[derive(Debug, Clone, Default, From, Deref)]
pub struct Theme(pub sleet::Theme);

impl stylesheet::application::DevAuto for Theme {}
impl stylesheet::pane_grid::DevAuto for Theme {}
impl stylesheet::scrollable::DevAuto for Theme {}

pub mod button {
    use super::*;

    use iced::widget::button;

    #[derive(Debug, Clone, Default)]
    pub enum Button {
        #[default]
        Default,
        Tab(pane::TabButtonStyle),
        SideBar(side_bar::ButtonStyle),
        Item(item::Style),
    }

    impl stylesheet::button::StyleSheet for Theme {
        type Style = Button;

        fn active(&self, style: &Self::Style) -> button::Appearance {
            match style {
                Self::Style::Tab(t) => t.active(self),
                Self::Style::SideBar(t) => t.active(self),
                Self::Style::Item(t) => t.active(self),
                _ => <sleet::Theme as stylesheet::button::StyleSheet>::active(
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
                _ => <sleet::Theme as stylesheet::button::StyleSheet>::hovered(
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
                _ => <sleet::Theme as stylesheet::button::StyleSheet>::pressed(
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
        /// A box drawn on the base color.
        OnBase,
        /// A box with a less emphasized color.
        Weak,
    }

    impl stylesheet::container::StyleSheet for Theme {
        type Style = Container;

        fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
            let palette = self.palette();

            match style {
                Container::Default => Default::default(),
                Container::OnBase => iced::widget::container::Appearance {
                    background: palette.primary.base.on_base.into(),
                    ..Default::default()
                },
                Container::Weak => iced::widget::container::Appearance {
                    background: palette.primary.weak.base.into(),
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

    impl stylesheet::svg::StyleSheet for Theme {
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

    impl stylesheet::text::StyleSheet for Theme {
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
    use sleet::fonts::font;

    /// More neutral font for primary contents.
    #[font(family = "Roboto")]
    pub enum Content {
        #[font(variant = "Regular")]
        #[font(source = "../assets/static/fonts/Roboto/Roboto-Regular.ttf")]
        Regular,

        // Not really the same family but close enough
        #[font(variant = "Mono Bold")]
        #[font(source = "../assets/static/fonts/Roboto/RobotoMono-Bold.ttf")]
        MonoBold,

        #[font(variant = "Mono Bold Italic")]
        #[font(source = "../assets/static/fonts/Roboto/RobotoMono-BoldItalic.ttf")]
        MonoBoldItalic,
    }

    /// More distinct font for secondary UI elements.
    #[font(family = "Sofia Sans")]
    pub enum UI {
        #[font(variant = "Regular")]
        #[font(source = "../assets/static/fonts/Sofia_Sans/SofiaSans-Regular.ttf")]
        Regular,

        #[font(variant = "Black")]
        #[font(source = "../assets/static/fonts/Sofia_Sans/SofiaSans-Black.ttf")]
        Black,
    }
}
