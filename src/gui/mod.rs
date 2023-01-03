//! The GUI app.

use iced::widget::row;
use iced::Application;
use iced::{executor, Command, Length};

use crate::config::Config;

pub mod pane;
use pane::Pane;

pub mod theme;
pub use theme::Theme;

pub mod tab;
pub use tab::Tab;

pub mod icons;
pub mod item;

pub mod side_bar;
pub use side_bar::SideBar;

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
    /// The side bar.
    side_bar: SideBar,
    /// Current configurations.
    config: Config,
}

impl Application for Gui {
    type Executor = executor::Default;
    type Flags = Config;
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let pane = Pane::new(Tab::new().unwrap());
        let pane_area = pane::Area::new(pane);

        let side_bar = SideBar {
            default: flags.side_bar.clone(),
            bookmarks: flags.bookmarks.clone(),
        };

        (
            Self {
                config: flags,
                side_bar,
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
        let side_bar = self
            .side_bar
            .view(|_| false)
            .unwrap()
            .map(Message::PaneArea);
        let pane_area = self.pane_area.view().unwrap().map(Message::PaneArea);

        row!(side_bar, pane_area)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
