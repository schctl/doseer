use sleet_font_macro::font;

#[font(family = "Roboto")]
pub enum Family {
    #[font(variant = "Regular")]
    #[font(source = "./data/dummy.ttf")]
    Regular,

    #[font(variant = "Bold")]
    #[font(source = "./data/dummy-bold.ttf")]
    Bold,
}

#[test]
fn test_names() {
    let regular = Family::Regular;
    let font: iced::Font = regular.into();

    match font {
        iced::Font::External { name, .. } => {
            assert_eq!(name, "Roboto Regular")
        }
        _ => panic!("did not construct font correctly"),
    }

    let bold = Family::Bold;
    let font: iced::Font = bold.into();

    match font {
        iced::Font::External { name, .. } => {
            assert_eq!(name, "Roboto Bold")
        }
        _ => panic!("did not construct font correctly"),
    }
}

#[test]
fn test_bytes() {
    let regular = Family::Regular;
    let font: iced::Font = regular.into();

    match font {
        iced::Font::External { bytes, .. } => {
            assert_eq!(bytes, &[0x01, 0x02, 0x03, 0x04])
        }
        _ => panic!("did not construct font correctly"),
    }

    let bold = Family::Bold;
    let font: iced::Font = bold.into();

    match font {
        iced::Font::External { bytes, .. } => {
            assert_eq!(bytes, &[0x05, 0x06, 0x07, 0x08])
        }
        _ => panic!("did not construct font correctly"),
    }
}
