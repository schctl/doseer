//! Defines the panel widget wrapper.

use std::marker::PhantomData;
use std::ops::Deref;

use iced_native::renderer;
use iced_native::widget::container;
use iced_native::widget::pane_grid::{self, PaneGrid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentType {
    Main,
    Panel,
}

impl ContentType {
    #[inline]
    pub fn is_main(&self) -> bool {
        *self == Self::Main
    }

    #[inline]
    pub fn is_panel(&self) -> bool {
        !self.is_main()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PanelPosition {
    #[default]
    Left,
    Right,
}

/// Panel wrapper state.
#[derive(Debug, Clone)]
pub struct State<Panel, Content> {
    internal: pane_grid::State<ContentType>,

    panel: pane_grid::Pane,
    content: pane_grid::Pane,

    _p0: PhantomData<Panel>,
    _p1: PhantomData<Content>,
}

impl<Panel, Content> State<Panel, Content> {
    pub fn new(panel_pos: PanelPosition) -> Self {
        // A vertical split will position the new pane on the right side.
        // So if we want our content to be on the right, it must be added to pane state
        // after the panel and vice-versa.
        let (first, second) = match panel_pos {
            PanelPosition::Left => (ContentType::Panel, ContentType::Main),
            PanelPosition::Right => (ContentType::Main, ContentType::Panel),
        };

        // Create first pane
        let (mut internal, first_id) = pane_grid::State::new(first);

        // Split vertically and add second pane to the right
        let (second_id, _) = internal
            .split(pane_grid::Axis::Vertical, &first_id, second)
            .unwrap();

        // Finally, resolve panel and content IDs
        let (side, content) = if first.is_panel() {
            (first_id, second_id)
        } else {
            (second_id, first_id)
        };

        Self {
            internal,
            panel: side,
            content,
            _p0: PhantomData,
            _p1: PhantomData,
        }
    }

    /// Swap the panel position.
    pub fn swap(&mut self) {
        self.internal.swap(&self.panel, &self.content);
    }
}

impl<Panel, Content> Deref for State<Panel, Content> {
    type Target = pane_grid::State<ContentType>;

    fn deref(&self) -> &Self::Target {
        &self.internal
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
    pub fn new<Panel, Content>(
        state: &'a State<Panel, Content>,
        panel: impl Fn() -> pane_grid::Content<'a, Message, Renderer>,
        content: impl Fn() -> pane_grid::Content<'a, Message, Renderer>,
    ) -> Self {
        let base = PaneGrid::new(&state.internal, |_, content_type, _| match content_type {
            ContentType::Panel => (panel)(),
            ContentType::Main => (content)(),
        });

        Self { base }
    }

    /// Returns the underlying [`PaneGrid`].
    #[inline]
    pub fn into_inner(self) -> PaneGrid<'a, Message, Renderer> {
        self.base
    }
}

impl<'a, Message, Renderer> From<Panelled<'a, Message, Renderer>>
    for PaneGrid<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: pane_grid::StyleSheet + container::StyleSheet,
{
    #[inline]
    fn from(value: Panelled<'a, Message, Renderer>) -> Self {
        value.into_inner()
    }
}
