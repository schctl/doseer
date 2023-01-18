//! Icon dictionary.

use iced::widget::svg::Handle;
use iced::widget::Svg;

const DIRECTORY: &[u8] = include_bytes!("../../res/static/icons/folder.svg");
const PLUS: &[u8] = include_bytes!("../../res/static/icons/plus.svg");
const CROSS: &[u8] = include_bytes!("../../res/static/icons/cross.svg");

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    Directory,
    Plus,
    Cross,
}

impl Icon {
    pub fn svg(&self) -> Svg<crate::gui::Renderer> {
        let handle = match self {
            Self::Directory => Handle::from_memory(DIRECTORY),
            Self::Plus => Handle::from_memory(PLUS),
            Self::Cross => Handle::from_memory(CROSS),
        };

        Svg::new(handle)
    }
}
