//! Grid with a flexbox layout.

use std::cmp::min;

pub use iced_lazy::responsive;
use iced_native::layout::Limits;
use iced_native::widget::{tree, Operation, Tree};
use iced_native::{
    event, layout, mouse, touch, Clipboard, Element, Event, Length, Point, Rectangle, Shell, Size,
    Widget,
};
use iced_native::{overlay, renderer};

use super::{direction, Order};

/// Local state of this grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    empty_space_clicked: bool,
}

impl State {
    /// Creates a new [`State`].
    pub fn new() -> Self {
        Self::default()
    }
}

/// A linearly-populated grid with a fixed cell size.
#[must_use]
pub struct Uniform<'a, Message, Renderer> {
    /// X-axis population direction.
    pop_x: direction::Horizontal,
    /// Y-axis population direction.
    pop_y: direction::Vertical,
    /// Overall population order.
    order: Order,

    /// Width of the whole grid widget.
    width: Length,
    /// Height of the whole grid widget.
    height: Length,
    /// Cell dimensions.
    cell: Size,

    /// X-axis spacing.
    spacing_x: f32,
    /// Y-axis spacing.
    spacing_y: f32,
    /// Allow spacing to be increased if possible.
    allow_more_spacing: bool,

    /// A message to emit if empty space was clicked.
    on_empty_click: Option<Message>,

    /// Contents of the grid.
    contents: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Uniform<'a, Message, Renderer> {
    /// Construct a new linear grid with the given contents.
    ///
    /// The grid will be populated will elements in the same order as the given iterator.
    #[inline]
    pub fn new(contents: impl Iterator<Item = Element<'a, Message, Renderer>>, cell: Size) -> Self {
        Self {
            pop_x: Default::default(),
            pop_y: Default::default(),
            order: Default::default(),
            width: Length::Shrink,
            height: Length::Shrink,
            cell,
            spacing_x: 0.0,
            spacing_y: 0.0,
            allow_more_spacing: false,
            on_empty_click: None,
            contents: contents.collect(),
        }
    }

    /// Sets the horizontal population order of the grid.
    pub const fn pop_x(mut self, dir: direction::Horizontal) -> Self {
        self.pop_x = dir;
        self
    }

    /// Sets the vertical population order of the grid.
    pub const fn pop_y(mut self, dir: direction::Vertical) -> Self {
        self.pop_y = dir;
        self
    }

    /// Sets the overall population order of the grid.
    pub const fn pop_order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }

    /// Sets the width of the grid.
    pub const fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the grid.
    pub const fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the horizontal spacing _between_ the cells of the grid.
    pub const fn spacing_x(mut self, units: u16) -> Self {
        self.spacing_x = units as f32;
        self
    }

    /// Sets the vertical spacing _between_ the cells of the grid.
    pub const fn spacing_y(mut self, units: u16) -> Self {
        self.spacing_y = units as f32;
        self
    }

    /// Sets the width of each cell.
    pub const fn cell_width(mut self, units: u16) -> Self {
        self.cell.width = units as f32;
        self
    }

    /// Sets the width of each cell.
    pub const fn cell_height(mut self, units: u16) -> Self {
        self.cell.height = units as f32;
        self
    }

    /// Allow spacing on the main axis to be increased if possible.
    pub const fn allow_more_spacing(mut self, allow: bool) -> Self {
        self.allow_more_spacing = allow;
        self
    }

    /// Emit a message if empty space between cells was clicked.
    pub fn on_empty_click(mut self, message: Message) -> Self {
        self.on_empty_click = Some(message);
        self
    }
}

impl<'a, Renderer, Message> Widget<Message, Renderer> for Uniform<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        self.contents.iter().map(|e| Tree::new(e)).collect()
    }

    fn diff(&self, state: &mut Tree) {
        state.diff_children(&self.contents);
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let total_size = limits.max();

        // -- Initial calculations --

        // Calculate number of rows and columns
        let max_cols = ((total_size.width + self.spacing_x) / (self.cell.width + self.spacing_x))
            .floor() as usize;
        let max_rows = ((total_size.height + self.spacing_y) / (self.cell.height + self.spacing_y))
            .floor() as usize;

        // Calculate dynamic spacing if any
        let (mut spacing_x, mut spacing_y) = (self.spacing_x, self.spacing_y);

        if self.allow_more_spacing {
            match self.order {
                // Allocate any extra space to more spacing
                Order::Horizontal => {
                    let remaining_space =
                        (max_cols as f32).mul_add(-self.cell.width, total_size.width);
                    spacing_x = (remaining_space / ((max_cols - 1) as f32)).max(self.spacing_x);
                }
                Order::Vertical => {
                    let remaining_space =
                        (max_rows as f32).mul_add(-self.cell.height, total_size.height);
                    spacing_y = (remaining_space / ((max_rows - 1) as f32)).max(self.spacing_y);
                }
            }
        }

        // --- Layout cells ---

        let indexes = |idx| match self.order {
            Order::Horizontal => match self.pop_x {
                direction::Horizontal::LeftToRight => (idx / max_cols, idx % max_cols),
                direction::Horizontal::RightToLeft => (idx / max_cols, max_cols - (idx % max_cols)),
            },
            Order::Vertical => match self.pop_y {
                direction::Vertical::TopToBottom => (idx % max_rows, idx / max_rows),
                direction::Vertical::BottomToTop => (max_rows - (idx % max_rows), idx / max_rows),
            },
        };

        let child_limits = Limits::new(Size::ZERO, self.cell);

        let children = (0..self.contents.len())
            .map(|idx| {
                let (row_idx, col_idx) = (indexes)(idx);

                let point = Point::new(
                    col_idx as f32 * (self.cell.width + spacing_x),
                    row_idx as f32 * (self.cell.height + spacing_y),
                );

                let mut child = self.contents[idx]
                    .as_widget()
                    .layout(renderer, &child_limits);
                child.move_to(point);

                child
            })
            .collect::<Vec<_>>();

        // --- Calculate bounded size ---

        let (last_row, last_col) = match self.order {
            Order::Horizontal => {
                let row = children.len() / max_cols;

                // hug contents
                let col = match self.pop_x {
                    direction::Horizontal::LeftToRight => min(self.contents.len(), max_cols),
                    direction::Horizontal::RightToLeft => max_cols,
                } - 1;

                (row, col)
            }
            Order::Vertical => {
                let col = children.len() / max_rows;

                // hug contents
                let row = match self.pop_y {
                    direction::Vertical::TopToBottom => min(self.contents.len(), max_rows),
                    direction::Vertical::BottomToTop => max_rows,
                } - 1;

                (row, col)
            }
        };

        let size = Size {
            width: (last_col as f32).mul_add(self.cell.width + spacing_x, self.cell.width),
            height: (last_row as f32).mul_add(self.cell.height + spacing_y, self.cell.height),
        };

        layout::Node::with_children(size, children)
    }

    // ˅ All of these are pretty much copied exactly from `Row`'s implementation ˅

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        for ((child, state), layout) in self
            .contents
            .iter()
            .zip(&state.children)
            .zip(layout.children())
        {
            child.as_widget().draw(
                state,
                renderer,
                theme,
                style,
                layout,
                cursor_position,
                viewport,
            );
        }
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, &mut |operation| {
            self.contents
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                });
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        // Let contents handle event first
        if event::Status::Captured
            == self
                .contents
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
                .map(|((child, state), layout)| {
                    child.as_widget_mut().on_event(
                        state,
                        event.clone(),
                        layout,
                        cursor_position,
                        renderer,
                        clipboard,
                        shell,
                    )
                })
                .fold(event::Status::Ignored, event::Status::merge)
        {
            return event::Status::Captured;
        }

        match event {
            // Handle empty space click
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let bounds = layout.bounds();

                if bounds.contains(cursor_position) {
                    let state = tree.state.downcast_mut::<State>();

                    let mut in_child = false;

                    for child in layout.children() {
                        if child.bounds().contains(cursor_position) {
                            in_child = true;
                            break;
                        }
                    }

                    if !in_child {
                        state.empty_space_clicked = true;
                        return event::Status::Captured;
                    }
                }
            }
            // Handle mouse release in empty space
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                if state.empty_space_clicked {
                    if let Some(message) = self.on_empty_click.clone() {
                        shell.publish(message);
                        state.empty_space_clicked = false;
                    }
                    return event::Status::Captured;
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.contents
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget().mouse_interaction(
                    state,
                    layout,
                    cursor_position,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn overlay<'overlay>(
        &'overlay mut self,
        state: &'overlay mut Tree,
        layout: iced_native::Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'overlay, Message, Renderer>> {
        overlay::from_children(&mut self.contents, state, layout, renderer)
    }
}

impl<'a, Message, Renderer> From<Uniform<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + renderer::Renderer,
{
    #[inline]
    fn from(value: Uniform<'a, Message, Renderer>) -> Self {
        Element::new(value)
    }
}

/// Construct a new [`Uniform`] grid.
#[inline]
pub fn uniform<'a, Message, Renderer>(
    contents: impl Iterator<Item = Element<'a, Message, Renderer>>,
    cell: Size,
) -> Uniform<'a, Message, Renderer> {
    Uniform::new(contents, cell)
}
