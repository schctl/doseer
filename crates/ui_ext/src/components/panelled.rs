//! Convenience wrapper around [`pane_grid`] to make handling of just two panes more ergonomic.

use derive_more::{Deref, DerefMut};
use iced_native::widget::container;
use iced_native::widget::pane_grid::Split;
pub use iced_native::widget::pane_grid::{self, Pane, PaneGrid};
use iced_native::{renderer, Element};

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct InternalState<T> {
    /// The state of the internal contents.
    #[deref]
    #[deref_mut]
    pub base: T,
    /// Associated pane grid ID.
    pub id: Pane,
}

/// Contains the state as well as other information to manipulate the pane.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct PositionData<T> {
    /// The state of the internal contents.
    #[deref]
    #[deref_mut]
    pub state: InternalState<T>,
    /// Associated split.
    pub split: Split,
    /// Position of the panel.
    pub position: PanelPosition,
}

/// The type of pane contained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentType {
    Main,
    Panel,
}

/// Position of the panel relative to the main contents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PanelPosition {
    #[default]
    Left,
    Right,
}

impl PanelPosition {
    #[inline]
    pub const fn inverse(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

/// Panel wrapper state.
#[derive(Debug, Clone)]
pub struct State<Panel, Content> {
    pub internal: pane_grid::State<ContentType>,
    content: InternalState<Content>,
    panel: Option<PositionData<Panel>>,
}

impl<Panel, Content> State<Panel, Content> {
    /// Create a new panelled widget with the provided content.
    pub fn new(content: Content) -> Self {
        let (internal, id) = pane_grid::State::new(ContentType::Main);
        let content = InternalState { base: content, id };

        Self {
            internal,
            content,
            panel: None,
        }
    }

    /// Add a panel at the given position.
    ///
    /// Returns [`None`] if either there is already a panel, or the split didn't work.
    pub fn add_panel(&mut self, panel: Panel, position: PanelPosition) -> Option<()> {
        if self.panel.is_some() {
            return None;
        }

        if let Some((id, split)) = self.internal.split(
            pane_grid::Axis::Vertical,
            &self.content.id,
            ContentType::Panel,
        ) {
            self.panel = Some(PositionData {
                state: InternalState { base: panel, id },
                split,
                position,
            });

            // A vertical split will position the new pane on the right side.
            // So if we want our content to be on the right, they must be swapped
            if position == PanelPosition::Left {
                self.swap();
            }

            return Some(());
        };

        None
    }

    /// Close the panel.
    ///
    /// Returns [`None`] if there is no panel.
    #[inline]
    pub fn close_panel(&mut self) -> Option<Panel> {
        self.panel.take().map(|panel| {
            self.internal.close(&panel.state.id).unwrap();
            panel.state.base
        })
    }

    #[inline]
    pub fn panel(&self) -> Option<&InternalState<Panel>> {
        self.panel.as_ref().map(|d| &d.state)
    }

    #[inline]
    pub fn panel_mut(&mut self) -> Option<&mut InternalState<Panel>> {
        self.panel.as_mut().map(|d| &mut d.state)
    }

    #[inline]
    pub fn content(&self) -> &InternalState<Content> {
        &self.content
    }

    #[inline]
    pub fn content_mut(&mut self) -> &mut InternalState<Content> {
        &mut self.content
    }

    /// Swap the panel position.
    ///
    /// Returns [`None`] if there is no panel.
    #[inline]
    pub fn swap(&mut self) -> Option<()> {
        self.panel.as_mut().map(|panel| {
            self.internal.swap(&panel.id, &self.content.id);
            panel.position = panel.position.inverse();
        })
    }

    /// Resize the panel. The provided ratio is that of the panel to the contents.
    ///
    /// Returns [`None`] if there is no panel.
    pub fn resize_panel(&mut self, ratio: f32) -> Option<()> {
        self.panel.as_mut().map(|p| {
            self.internal.resize(
                &p.split,
                if p.position == PanelPosition::Right {
                    ratio
                } else {
                    1.0 - ratio
                },
            );
        })
    }
}

/// A wrapper widget with an attached secondary panel.
pub struct Panelled<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: pane_grid::StyleSheet + container::StyleSheet,
{
    base: PaneGrid<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Panelled<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: pane_grid::StyleSheet + container::StyleSheet,
{
    /// Create a new widget with the provided panel and content.
    pub fn new<Panel, PanelledContent>(
        state: &'a State<Panel, PanelledContent>,
        panel: impl Fn(&'a InternalState<Panel>) -> pane_grid::Content<'a, Message, Renderer>,
        content: impl Fn(
            &'a InternalState<PanelledContent>,
        ) -> pane_grid::Content<'a, Message, Renderer>,
    ) -> Self {
        let base = PaneGrid::new(&state.internal, |_, content_type, _| match content_type {
            ContentType::Panel => (panel)(state.panel.as_ref().unwrap()),
            ContentType::Main => (content)(&state.content),
        });

        Self { base }
    }

    /// Returns the underlying [`PaneGrid`].
    #[inline]
    pub fn into_inner(self) -> PaneGrid<'a, Message, Renderer> {
        self.base
    }
}

impl<'a, Message, Renderer> From<Panelled<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
    Renderer::Theme: pane_grid::StyleSheet + container::StyleSheet,
{
    #[inline]
    fn from(value: Panelled<'a, Message, Renderer>) -> Self {
        value.into_inner().into()
    }
}
