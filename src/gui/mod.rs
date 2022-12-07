//! The GUI app.

use iced::widget::{pane_grid, Container, Button, Text};
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
    /// All held tabs.
    tabs: Vec<Tab>,
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
        let tabs = vec![Tab {
            id: 0,
            location: crate::dirs::BASE.home_dir().to_owned(),
            scroll: tab::ScrollableState::default(),
        }];

        let (panes, _) = pane_grid::State::new(Pane {
            id: 0,
            tabs: vec![0],
            focused: 0,
        });

        (
            Self {
                config: flags,
                tabs,
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
            let thing = format!(
                "{:?}",
                self.tabs[state.focused].location
            );

            pane_grid::Content::new(Button::new(Text::new(thing)))
        });

        Container::new(pane_grid).width(Length::Fill).height(Length::Fill).into()
    }
}
