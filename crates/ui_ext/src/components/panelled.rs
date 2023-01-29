//! A wrapper widget to make handling an attached panel easier.

use iced_native::widget::container;
use iced_native::widget::pane_grid::{Pane, PaneGrid, Split};
use iced_native::{renderer, Element};

pub use iced_native::widget::pane_grid;

/// The type of pane contained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ContentType {
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

/// Internal state of a panelled widget.
pub struct State {
    internal: pane_grid::State<ContentType>,
    panel_id: Pane,
    content_id: Pane,
    split: Split,
    pub position: PanelPosition,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    /// Initialize a new panel state with default values.
    pub fn new() -> Self {
        let (mut internal, content_id) = pane_grid::State::new(ContentType::Panel);

        let (panel_id, split) = internal
            .split(pane_grid::Axis::Vertical, &content_id, ContentType::Main)
            .unwrap();

        Self {
            internal,
            panel_id,
            content_id,
            split,
            position: PanelPosition::Left,
        }
    }

    /// Swap the panel position.
    pub fn swap(&mut self) {
        self.internal.swap(&self.panel_id, &self.content_id)
    }

    /// Resize the panel. The provided ratio is that of the panel to the contents.
    pub fn resize(&mut self, ratio: f32) {
        self.internal.resize(
            &self.split,
            if self.position == PanelPosition::Right {
                1.0 - ratio
            } else {
                ratio
            },
        );
    }
}

// --- Unpanelled Wrapper ---

/// Indicates that the contents have no attached panel.
pub struct Unpanelled<'a, Message, Renderer> {
    view: Box<dyn Fn() -> Element<'a, Message, Renderer> + 'a>,
}

impl<'a, Message, Renderer> Unpanelled<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: pane_grid::StyleSheet + container::StyleSheet,
{
    /// Create a new [`Unpanelled`] widget.
    ///
    /// A panel can be added to this later.
    pub fn new(view: impl Fn() -> Element<'a, Message, Renderer> + 'a) -> Self {
        Self {
            view: Box::new(view),
        }
    }

    /// Add a panel to this widget.
    pub fn panel(
        self,
        state: &'a State,
        view: impl Fn(PanelPosition) -> Element<'a, Message, Renderer>,
    ) -> WithPanel<'a, Message, Renderer> {
        let base = PaneGrid::new(&state.internal, move |_, content_type, _| {
            let content = match content_type {
                ContentType::Panel => (view)(state.position),
                ContentType::Main => (self.view)(),
            };

            pane_grid::Content::new(content)
        });

        WithPanel { base }
    }
}

// --- Hope you like reading generics :) ---

impl<'a, Message, Renderer> From<Unpanelled<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + 'a,
    Renderer::Theme: pane_grid::StyleSheet + container::StyleSheet,
{
    fn from(value: Unpanelled<'a, Message, Renderer>) -> Self {
        (value.view)()
    }
}

/// Indicates that the contents have an attached panel.
pub struct WithPanel<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: pane_grid::StyleSheet + container::StyleSheet,
{
    base: PaneGrid<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> WithPanel<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: pane_grid::StyleSheet + container::StyleSheet,
{
    pub fn into_inner(self) -> PaneGrid<'a, Message, Renderer> {
        self.base
    }
}

impl<'a, Message, Renderer> From<WithPanel<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + 'a,
    Renderer::Theme: pane_grid::StyleSheet + container::StyleSheet,
{
    fn from(value: WithPanel<'a, Message, Renderer>) -> Self {
        Self::from(value.base)
    }
}

/// Create a new [`Unpanelled`] widget.
pub fn unpanelled<'a, Message, Renderer>(
    view: impl Fn() -> Element<'a, Message, Renderer> + 'a,
) -> Unpanelled<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: pane_grid::StyleSheet + container::StyleSheet,
{
    Unpanelled::new(view)
}
