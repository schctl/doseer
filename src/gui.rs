//! The GUI app.

use m7_core::config::Config;
use m7_core::path::PathWrap;
use m7_ui_ext::components::panelled::{self, Panelled};

use iced::{executor, Application, Command, Length};
use sleet::stylesheet::Wrap;

use crate::pane::{self, Pane};
use crate::{SideBar, Tab, Theme};

/// Shorthand for an iced element generic over some message.
pub type Renderer = iced::Renderer<Wrap<Theme>>;
pub type Element<'a, T> = iced::Element<'a, T, Renderer>;

#[derive(Debug, Clone)]
pub enum Message {
    PaneArea(pane::area::Message),
    ResizeMain(panelled::pane_grid::ResizeEvent),
    IcedEvent(iced::Event),
}

/// The UI state.
pub struct Gui {
    /// File pane grid area.
    main_area: panelled::State<SideBar, pane::Area>,
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

        let mut main_area = panelled::State::new(pane_area);
        main_area.add_panel(side_bar, panelled::PanelPosition::Left);
        main_area.resize_panel(0.2);

        (
            Self {
                main_area,
                _config: flags,
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
            Message::PaneArea(m) => self.main_area.content_mut().update(m).unwrap(),
            Message::ResizeMain(m) => self.main_area.internal.resize(&m.split, m.ratio),
            _ => {}
        }

        Command::none()
    }

    fn theme(&self) -> Self::Theme {
        Wrap(sleet::style::Theme::Catppuccin(sleet::style::catppuccin::Variant::Mocha).into())
    }

    fn view(&self) -> Element<Self::Message> {
        Panelled::new(
            &self.main_area,
            |panel| {
                panel
                    .view(|path| {
                        self.main_area.content().base.focused().location().as_ref() == path
                    })
                    .unwrap()
                    .map(Message::PaneArea)
                    .into()
            },
            |content| content.view().map(Message::PaneArea).into(),
        )
        .into_inner()
        .width(Length::Fill)
        .height(Length::Fill)
        .on_resize(16, Message::ResizeMain)
        .into()
    }
}
