//! The actual *content* to be displayed by a tab - directory contents, etc.

use std::path::Path;

use doseer_core::dirs;
use doseer_core::path::PathWrap;
use doseer_iced_ext::widgets::grid::uniform;

use iced::widget::scrollable::Properties;
use iced::widget::{container, scrollable, Component};
use iced::Length;

use crate::gui::Element;
use crate::{gui, item};

/// Create location content state from predefined state.
#[inline]
pub const fn content(state: &State) -> Content {
    Content::new(state)
}

/// Externally managed content state.
#[derive(Debug)]
pub struct State {
    /// Directory stack.
    pub stack: Vec<PathWrap>,
    /// Contents of the current location.
    pub contents: dirs::Contents,
}

impl State {
    /// Default tab content.
    #[inline]
    pub fn new() -> anyhow::Result<Self> {
        Self::new_with(dirs::BASE.home_dir())
    }

    /// Tab content with a specified location.
    #[inline]
    pub fn new_with<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        Ok(Self {
            contents: dirs::Contents::new(path)?,
            stack: vec![],
        })
    }

    /// Get the location this content points to.
    #[inline]
    pub fn location(&self) -> &PathWrap {
        self.contents.location()
    }

    /// Change this content to point to a new location.
    pub fn update_location<P: AsRef<Path>>(&mut self, new: P) -> anyhow::Result<()> {
        self.contents = dirs::Contents::new(new)?;
        Ok(())
    }
}

/// Internally managed content state.
#[derive(Debug, Default, Clone)]
pub struct InternalState {
    /// The currently selected item.
    selected: Option<PathWrap>,
}

impl InternalState {
    /// Check if an item is currently selected.
    fn is_selected<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(selected) = &self.selected {
            if selected.as_ref() == path.as_ref() {
                return true;
            }
        }

        false
    }
}

/// Internal messages.
#[derive(Debug, Clone)]
pub enum Event {
    Item(item::Message),
}

/// Content component.
pub struct Content<'app> {
    state: &'app State,
}

impl<'app> Content<'app> {
    /// Create location content state from predefined state.
    #[inline]
    pub const fn new(state: &'app State) -> Self {
        Self { state }
    }
}

impl<'app> Component<super::Event, gui::Renderer> for Content<'app> {
    type State = InternalState;
    type Event = Event;

    fn update(
        &mut self,
        internal_state: &mut Self::State,
        event: Self::Event,
    ) -> Option<super::Event> {
        match event {
            Event::Item(i) => match i {
                item::Message::Select(s) => {
                    if internal_state.is_selected(&s) {
                        return Some(super::Event::Open(s));
                    } else {
                        internal_state.selected = Some(s);
                    }
                }
                item::Message::Deselect => internal_state.selected = None,
            },
        }

        None
    }

    fn view(&self, internal_state: &Self::State) -> Element<'_, Self::Event> {
        // weird lifetime shenanigans without ownership
        let internal_state = internal_state.clone();

        uniform::responsive(move |_| {
            let grid = uniform(
                self.state.contents.contents().iter().map(|path| {
                    container(
                        item::view(
                            path.clone(),
                            if internal_state.is_selected(path) {
                                item::Style::Selected
                            } else {
                                item::Style::Default
                            },
                        )
                        .map(Event::Item),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
                }),
                item::DIMENSIONS,
            )
            .spacing_x(12)
            .spacing_y(12)
            .allow_more_spacing(true)
            .on_empty_click(Event::Item(item::Message::Deselect));

            scrollable(container(grid).padding([0.0, 16.0]))
                .direction(scrollable::Direction::Vertical(
                    Properties::new().width(5.6).scroller_width(5.0).margin(3.6),
                ))
                .into()
        })
        .into()
    }
}
