//! Icon dictionary.

use iced::widget::svg::Handle;
use iced::widget::Svg;

const DIRECTORY: &[u8] = include_bytes!("../../res/folder.svg");
const PLUS: &[u8] = include_bytes!("../../res/plus.svg");

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    Directory,
    Plus,
}

impl Icon {
    pub fn svg(&self) -> Svg<crate::gui::Renderer> {
        let handle = match self {
            Self::Directory => Handle::from_memory(DIRECTORY),
            Self::Plus => Handle::from_memory(PLUS),
        };

        Svg::new(handle)
    }
}
