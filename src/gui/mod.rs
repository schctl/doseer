//! The GUI app.

use iced::widget::Container;
use iced::Application;
use iced::{executor, Command, Length};

use crate::config::Config;

pub mod pane;
use pane::{Pane, Tab};

pub mod theme;
pub use theme::Theme;

/// Shorthand for an iced element generic over some message.
pub type Element<'a, T> = iced::Element<'a, T, iced::Renderer<Theme>>;

#[derive(Debug, Clone)]
pub enum Message {
    PaneArea(pane::area::Message),
    IcedEvent(iced::Event),
}

/// The UI state.
pub struct Gui {
    /// File pane grid area.
    pane_area: pane::Area,
    /// Current configurations.
    config: Config,
}

impl Application for Gui {
    type Executor = executor::Default;
    type Flags = Config;
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut pane = Pane::new(Tab::new().unwrap());
        pane.add_tab(Tab::new_with("/usr/lib").unwrap());

        let pane_area = pane::Area::new(pane);

        (
            Self {
                config: flags,
                pane_area,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("m7")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events().map(Message::IcedEvent)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::PaneArea(m) => self.pane_area.update(m).unwrap(),
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let pane_area = self.pane_area.view().unwrap().map(Message::PaneArea);

        Container::new(pane_area)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
