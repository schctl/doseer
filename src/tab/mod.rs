//! Tab widget.

use std::path::Path;

use doseer_core::path::PathWrap;

use iced::widget::pane_grid;
use iced::Command;
use iced_lazy::{component, Component};

use crate::gui::{self, Element};

use self::content::content;

pub mod content;
pub mod watcher;

/// Create tab widget from given state.
#[inline]
pub const fn tab(state: &State) -> Tab {
    Tab::new(state)
}

/// External messages.
#[derive(Debug, Clone)]
pub enum Message {
    /// Open this location in the current pane.
    Open(PathWrap),
    /// Update the contents of the current pane.
    Update,
    /// Failed to watch location.
    WatchFail,
}

/// Externally managed state.
#[derive(Debug)]
pub struct State {
    /// Pane grid managed state.
    pane_grid: pane_grid::State<content::State>,
    /// Focused pane.
    focused: pane_grid::Pane,
}

impl State {
    /// Default tab state.
    #[inline]
    pub fn new() -> anyhow::Result<Self> {
        let (pane_grid, focused) = pane_grid::State::new(content::State::new()?);

        Ok(Self { pane_grid, focused })
    }

    /// New tab state with specified location for the first pane.
    #[inline]
    pub fn new_with<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let (pane_grid, focused) = pane_grid::State::new(content::State::new_with(path)?);

        Ok(Self { pane_grid, focused })
    }

    /// Open a location in the current pane.
    pub fn open(&mut self, path: &PathWrap) -> anyhow::Result<Command<Message>> {
        if path.is_dir() {
            self.pane_grid
                .get_mut(&self.focused)
                .unwrap()
                .update_location(&path)?;
            return Ok(watcher::command(&path));
        }

        open::that(path.as_ref())?;
        Ok(Command::none())
    }

    /// Get the location of the current pane.
    #[inline]
    pub fn location(&self) -> &PathWrap {
        self.pane_grid.get(&self.focused).unwrap().location()
    }
}

/// Internal messages.
#[derive(Debug)]
pub enum Event {
    /// Open this location in the current pane.
    Open(PathWrap),
}

/// Tab component.
pub struct Tab<'app> {
    state: &'app State,
}

impl<'app> Tab<'app> {
    /// Create tab widget with given state.
    #[inline]
    pub const fn new(state: &'app State) -> Self {
        Self { state }
    }
}

impl<'app> Component<Message, gui::Renderer> for Tab<'app> {
    type State = ();
    type Event = Event;

    fn update(&mut self, _: &mut Self::State, event: Self::Event) -> Option<Message> {
        // TODO: handle pane grid events
        match event {
            Event::Open(o) => Some(Message::Open(o)),
        }
    }

    fn view(&self, _: &Self::State) -> Element<'_, Self::Event> {
        // TODO: top toolkit
        pane_grid::PaneGrid::new(&self.state.pane_grid, |_, content_state, _| {
            pane_grid::Content::new(component(content(content_state)))
        })
        .into()
    }
}
