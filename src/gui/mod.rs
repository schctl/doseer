//! The GUI app.

use iced::widget::{pane_grid, Button, Container, Text};
use iced::{executor, Command, Length};
use iced::{Application, Theme};

use crate::config::Config;

pub mod pane;
use pane::tab::{self, Tab};
use pane::Pane;

#[derive(Debug, Clone)]
pub enum Message {}

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
        let (panes, _) = pane_grid::State::new(Pane::new(Tab::new()));

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

    fn update(&mut self, _: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let pane_grid = pane_grid::PaneGrid::new(&self.panes, |pane, state, focused| {
            let thing = format!("{:?}", state.focused().location());

            pane_grid::Content::new(Button::new(Text::new(thing)))
        });

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
