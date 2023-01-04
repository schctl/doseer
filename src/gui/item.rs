//! A single item (file/folder/whatever) in a directory.

use std::path::Path;

use iced::widget::svg::Handle;
use iced::widget::{column, container, Svg, Text};

use crate::gui::{icons, Element};

pub fn view(path: &Path) -> Element<()> {
    let icon = Svg::new(Handle::from_memory(icons::DIRECTORY));

    let text = Text::new(path.as_os_str().to_string_lossy())
        .horizontal_alignment(iced::alignment::Horizontal::Center);

    container(
        column!(icon, text)
            .align_items(iced::Alignment::Center)
            .width(iced::Length::Fill),
    )
    .width(iced::Length::Fill)
    .into()
}
