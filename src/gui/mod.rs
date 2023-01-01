//! The GUI app.

use iced::widget::{pane_grid, Container};
use iced::Application;
use iced::{executor, Command, Length};

use crate::config::Config;

pub mod pane;
use pane::tab::{self, Tab};
use pane::Pane;

pub mod theme;
pub use theme::Theme;

#[derive(Debug, Clone)]
pub enum Message {
    PaneMessage(pane::Message, pane_grid::Pane),
    IcedEvent(iced::Event),
}

/// The UI state.
pub struct Gui {
    /// Current configurations.
    config: Config,
    /// Panegrid state.
    panes: pane_grid::State<Pane>,
    /// Focused pane.
    focused: usize,
}

impl Application for Gui {
    type Executor = executor::Default;
    type Flags = Config;
    type Message = Message;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut pane = Pane::new(Tab::new().unwrap());
        pane.add_tab(Tab::new_with("/usr/lib").unwrap());
        let (panes, _) = pane_grid::State::new(pane);

        let theme = iced::Theme::Dark;

        (
            Self {
                config: flags,
                panes,
                focused: 0,
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

    fn update(&mut self, m: Self::Message) -> Command<Self::Message> {
        match m {
            Message::PaneMessage(message, id) => {
                let pane = self.panes.panes.get_mut(&id).unwrap();
                pane.update(message).unwrap();
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Theme>> {
        let pane_grid = pane_grid::PaneGrid::new(&self.panes, |id, state, _focused| {
            pane_grid::Content::new(
                state
                    .view(pane::ViewOpts {
                        tab: tab::ViewOpts { columns: 6 },
                    })
                    .unwrap()
                    .map(move |m| Message::PaneMessage(m, id)),
            )
        });

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
