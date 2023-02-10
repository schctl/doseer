//! The GUI app.

use doseer_core::config::Config;
use doseer_ui_ext::components::panelled::{self, unpanelled};

use iced::{executor, Application, Command, Length};
use sleet::stylesheet::Wrap;

use crate::pane::{self, Pane};
use crate::{tab, SideBar, Tab, Theme};

/// Shorthand for an iced element generic over some message.
pub type Renderer = iced::Renderer<Wrap<Theme>>;
pub type Element<'a, T> = iced::Element<'a, T, Renderer>;

#[derive(Debug, Clone)]
pub enum Message {
    Pane(pane::Message),
    ResizeMain(panelled::pane_grid::ResizeEvent),
    IcedEvent(iced::Event),
}

/// The UI state.
pub struct Gui {
    /// Sidebar split tracker.
    split_state: panelled::State,
    /// Sidebar state.
    side_bar: SideBar,
    /// Pane state.
    pane: Pane,
}

impl Application for Gui {
    type Executor = executor::Default;
    type Flags = Config;
    type Message = Message;
    type Theme = Wrap<Theme>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut commands = vec![];

        let pane = Pane::new(Tab::new().unwrap());
        let side_bar = SideBar::new(flags.side_bar);

        let mut split_state = panelled::State::new();
        split_state.resize(0.2);

        // :/
        let tab_init_cmd = tab::watcher::command(pane.focused().location());
        commands.push(tab_init_cmd.map(|m| Message::Pane(pane::Message::Tab(m, None))));

        (
            Self {
                split_state,
                side_bar,
                pane,
            },
            Command::batch(commands),
        )
    }

    fn title(&self) -> String {
        String::from("doseer")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events().map(Message::IcedEvent)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let mut commands = vec![];

        match message {
            Message::Pane(m) => {
                let pane_cmd = self.pane.update(m).unwrap();
                commands.push(pane_cmd.map(Message::Pane));
            }
            Message::ResizeMain(m) => self.split_state.resize(m.ratio),
            _ => {}
        }

        Command::batch(commands)
    }

    fn theme(&self) -> Self::Theme {
        Wrap(sleet::Theme::Catppuccin(sleet::colorscheme::catppuccin::Variant::Mocha).into())
    }

    fn view(&self) -> Element<Self::Message> {
        unpanelled(|| self.pane.view().map(Message::Pane))
            // add side panel
            .panel(&self.split_state, |_| {
                self.side_bar
                    .view(|path| self.pane.focused().location().as_ref() == path)
                    .unwrap()
                    .map(Message::Pane)
            })
            // configure inner pane_grid
            .into_inner()
            .width(Length::Fill)
            .height(Length::Fill)
            .on_resize(16, Message::ResizeMain)
            .into()
    }
}
