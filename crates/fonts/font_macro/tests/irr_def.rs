//! Tests irregular definitions.

use sleet_font_macro::font;

pub mod some_random_path {
    pub mod with_2_depth {
        pub use iced::Font as _RandomFontName_;
    }
}

// -- Multiple font definitions --

#[font(
    family = "Roboto",
    // Custom font type
    font_typ = "some_random_path::with_2_depth::_RandomFontName_"
)]
pub enum Family0 {
    #[font(variant = "Regular")]
    #[font(source = "./data/dummy.ttf")]
    Regular,

    #[font(variant = "Bold")]
    #[font(source = "./data/dummy-bold.ttf")]
    Bold,
}

#[font(family = "Roboto", font_typ = "::iced::Font")]
pub enum Family1 {
    // Repeating Variant definitions
    #[font(variant = "Regular")]
    #[font(source = "./data/dummy.ttf")]
    Regular0,

    #[font(variant = "Regular")]
    #[font(source = "./data/dummy.ttf")]
    Regular1,

    #[font(variant = "Bold")]
    #[font(source = "./data/dummy-bold.ttf")]
    Bold,
}
