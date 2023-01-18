//! The GUI app.

use iced::widget::row;
use iced::{executor, Application, Command, Length};
use sleet::stylesheet::Wrap;

use m7_core::config::Config;
use m7_core::path::PathWrap;

use crate::pane::{self, Pane};
use crate::{SideBar, Tab, Theme};

/// Shorthand for an iced element generic over some message.
pub type Renderer = iced::Renderer<Wrap<Theme>>;
pub type Element<'a, T> = iced::Element<'a, T, Renderer>;

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
    _config: Config,
}

impl Application for Gui {
    type Executor = executor::Default;
    type Flags = Config;
    type Message = Message;
    type Theme = Wrap<Theme>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let pane = Pane::new(Tab::new().unwrap());
        let pane_area = pane::Area::new(pane);

        let side_bar = SideBar {
            default: flags
                .side_bar
                .iter()
                .map(|p| PathWrap::from_path(p).unwrap())
                .collect(),
            bookmarks: flags
                .bookmarks
                .iter()
                .map(|p| PathWrap::from_path(p).unwrap())
                .collect(),
        };

        (
            Self {
                _config: flags,
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
        let pane_area = self.pane_area.view().map(Message::PaneArea);

        row!(side_bar, pane_area)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}