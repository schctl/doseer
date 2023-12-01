//! Modified [`Row`](iced_widget::Row) implementation that allows for re-ordering content.

use iced_core::event::{self, Event};
use iced_core::layout::{self, Layout};
use iced_core::widget::{tree, Operation, Tree};
use iced_core::{
    mouse, overlay, renderer, touch, Alignment, Clipboard, Element, Length, Padding, Pixels,
    Rectangle, Shell, Vector, Widget,
};

use super::{set_available_cursor_position, Axis, Drag, State};

/// A container that distributes its contents horizontally.
pub struct Row<'a, Message, Renderer> {
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
    align_items: Alignment,
    children: Vec<Element<'a, Message, Renderer>>,
    /// Message producer for when elements are reordered.
    on_reorder: Option<Box<dyn Fn(usize, usize) -> Message + 'a>>,
}

impl<'a, Message, Renderer> Row<'a, Message, Renderer> {
    /// Creates an empty [`Row`].
    pub fn new() -> Self {
        Self::with_children(Vec::new())
    }

    /// Creates a [`Row`] with the given elements.
    pub fn with_children(children: Vec<Element<'a, Message, Renderer>>) -> Self {
        Row {
            spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            align_items: Alignment::Start,
            children,
            on_reorder: None,
        }
    }

    /// Sets the horizontal spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self {
        self.spacing = amount.into().0;
        self
    }

    /// Sets the [`Padding`] of the [`Row`].
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Row`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Row`].
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the vertical alignment of the contents of the [`Row`] .
    pub fn align_items(mut self, align: Alignment) -> Self {
        self.align_items = align;
        self
    }

    /// Adds an [`Element`] to the [`Row`].
    pub fn push(mut self, child: impl Into<Element<'a, Message, Renderer>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Produce a message when two elements are swapped.
    pub fn on_reorder(mut self, f: impl Fn(usize, usize) -> Message + 'a) -> Self {
        self.on_reorder = Some(Box::new(f));
        self
    }
}

impl<'a, Message, Renderer> Default for Row<'a, Message, Renderer> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Row<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children)
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        layout::flex::resolve(
            layout::flex::Axis::Horizontal,
            renderer,
            &limits,
            self.padding,
            self.spacing,
            self.align_items,
            &self.children,
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.children
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                })
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        // --- The main part: Handle dragging ---

        let state = tree.state.downcast_mut::<State>();

        match event {
            // Begin dragging
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let bounds = layout.bounds();

                if let Some(cursor_position) = cursor.position() {
                    if bounds.contains(cursor_position) {
                        for (n, child) in layout.children().enumerate() {
                            let child_bounds = child.bounds();

                            // Begin dragging on a child if it contains the cursor
                            if child_bounds.contains(cursor_position) {
                                state.drag_state = Some(Drag {
                                    index: n,
                                    begun_at: cursor_position,
                                    currently_at: cursor_position,
                                });

                                break;
                            }
                        }
                    }
                }
            }
            // Quit dragging
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                state.drag_state = None;
            }
            // Perform dragging if begun
            Event::Mouse(mouse::Event::CursorMoved { position, .. })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if let Some(drag_state) = &mut state.drag_state {
                    drag_state.currently_at = position;

                    // The important part: handle reordering
                    let with = super::should_swap(layout.children(), drag_state, |p| p.x, Axis::x);

                    // Perform swap
                    if let Some((with, shift)) = with {
                        if let Some(on_reorder) = self.on_reorder.as_ref() {
                            let message = (on_reorder)(drag_state.index, with);
                            shell.publish(message);
                        }
                        drag_state.begun_at.x += shift;
                        drag_state.index = with;
                    }

                    // Continue dragging
                }
            }
            _ => {}
        }

        // If dragging is in progress, don't let children capture events
        if let Some(drag_state) = state.drag_state.as_ref() {
            if drag_state.currently_at != drag_state.begun_at {
                return event::Status::Captured;
            }
        }

        // ---   ---   ---        ---   ---   ---

        self.children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            })
            .fold(event::Status::Ignored, event::Status::merge)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        mut cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();

        // Process drag state
        let drag_state = match state.drag_state.as_ref() {
            Some(drag_state) => {
                // Trick children into thinking cursor is at a different position
                // This is so widgets don't process cursor positions as being hovered over them, etc.
                set_available_cursor_position(&mut cursor, drag_state.begun_at);
                drag_state
            }
            _ => &Drag::ZERO,
        };

        for (n, ((child, child_state), child_layout)) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .enumerate()
        {
            // Draw dragged element in another layer
            if n == drag_state.index {
                let bounds = layout.bounds();
                let child_bounds = child_layout.bounds();

                // Calculate maximum delta
                let mut delta = drag_state.currently_at - drag_state.begun_at;
                super::bind_delta(&mut delta, child_bounds, bounds);

                // Snap to x-axis
                let translation = Vector { x: delta.x, y: 0.0 };

                // Draw on next layer
                renderer.with_translation(translation, |renderer| {
                    renderer.with_layer(child_layout.bounds(), |renderer| {
                        child.as_widget().draw(
                            child_state,
                            renderer,
                            theme,
                            style,
                            child_layout,
                            cursor,
                            viewport,
                        )
                    })
                });

                continue;
            }

            child.as_widget().draw(
                child_state,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        overlay::from_children(&mut self.children, tree, layout, renderer)
    }
}

impl<'a, Message, Renderer> From<Row<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(row: Row<'a, Message, Renderer>) -> Self {
        Self::new(row)
    }
}
