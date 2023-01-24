//! Grid with a flexbox layout.

pub use iced_lazy::responsive;
use iced_native::layout::Limits;
use iced_native::widget::{Operation, Tree};
use iced_native::{
    event, layout, mouse, Clipboard, Element, Event, Length, Point, Rectangle, Shell, Size, Widget,
};
use iced_native::{overlay, renderer};

use super::{direction, Order};

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
    cell: Size<f32>,

    /// X-axis spacing.
    spacing_x: f32,
    /// Y-axis spacing.
    spacing_y: f32,
    /// Allow spacing to be increased if possible.
    allow_more_spacing: bool,

    /// Contents of the grid.
    contents: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Uniform<'a, Message, Renderer> {
    /// Construct a new linear grid with the given contents.
    ///
    /// The grid will be populated will elements in the same order as the given iterator.
    #[inline]
    pub fn new(
        contents: impl Iterator<Item = Element<'a, Message, Renderer>>,
        cell: Size<u16>,
    ) -> Self {
        Self {
            pop_x: Default::default(),
            pop_y: Default::default(),
            order: Default::default(),
            width: Length::Fill,
            height: Length::Fill,
            cell: Size {
                width: cell.width as f32,
                height: cell.height as f32,
            },
            spacing_x: 0.0,
            spacing_y: 0.0,
            contents: contents.collect(),
            allow_more_spacing: false,
        }
    }

    /// Sets the horizontal population order of the grid.
    pub fn pop_x(mut self, dir: direction::Horizontal) -> Self {
        self.pop_x = dir;
        self
    }

    /// Sets the vertical population order of the grid.
    pub fn pop_y(mut self, dir: direction::Vertical) -> Self {
        self.pop_y = dir;
        self
    }

    /// Sets the overall population order of the grid.
    pub fn pop_order(mut self, order: Order) -> Self {
        self.order = order;
        self
    }

    /// Sets the width of the grid.
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the grid.
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the horizontal spacing _between_ the cells of the grid.
    pub fn spacing_x(mut self, units: u16) -> Self {
        self.spacing_x = units as f32;
        self
    }

    /// Sets the vertical spacing _between_ the cells of the grid.
    pub fn spacing_y(mut self, units: u16) -> Self {
        self.spacing_y = units as f32;
        self
    }

    /// Sets the width of each cell.
    pub fn cell_width(mut self, units: u16) -> Self {
        self.cell.width = units as f32;
        self
    }

    /// Sets the width of each cell.
    pub fn cell_height(mut self, units: u16) -> Self {
        self.cell.height = units as f32;
        self
    }

    /// Allow spacing on the main axis to be increased if possible.
    pub fn allow_more_spacing(mut self, allow: bool) -> Self {
        self.allow_more_spacing = allow;
        self
    }
}

impl<'a, Renderer, Message> Widget<Message, Renderer> for Uniform<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
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
        let rows = ((total_size.width + self.spacing_x) / (self.cell.width + self.spacing_x))
            .floor() as u16;
        let cols = ((total_size.height + self.spacing_y) / (self.cell.height + self.spacing_y))
            .floor() as u16;

        // Calculate dynamic spacing if any
        let (mut spacing_x, mut spacing_y) = (self.spacing_x, self.spacing_y);

        if self.allow_more_spacing {
            match self.order {
                // Allocate any extra space to more spacing
                Order::Horizontal => {
                    let remaining_space = (total_size.width) - (rows as f32 * self.cell.width);
                    spacing_x = (remaining_space / ((rows - 1) as f32)).max(self.spacing_x);
                }
                Order::Vertical => {
                    let remaining_space = (total_size.height) - (cols as f32 * self.cell.height);
                    spacing_y = (remaining_space / ((cols - 1) as f32)).max(self.spacing_y);
                }
            }
        }

        // -- Accumulators --

        struct AxisData {
            /// Spacing along this axis
            spacing: f32,
            /// Index multiplication factor
            factor: i32,
            /// Current row/column
            /// - This is what should be tracked
            index: u16,
            /// Starting row/column
            reset: u16,
            /// Max row/column
            limiter: u16,
        }

        impl AxisData {
            pub fn increment(&mut self, other: &mut Self) -> bool {
                let new_self = (self.index as i32 + self.factor) as u16;

                // Extends the y-axis
                if new_self >= self.limiter {
                    // Extends the x-axis. Stop here.
                    if other.index + 1 > other.limiter {
                        return false;
                    }

                    self.index = self.reset;
                    other.index = (other.index as i32 + other.factor) as u16;
                } else {
                    self.index = new_self;
                }

                true
            }
        }

        let mut data_x = {
            let (reset, limiter) = match self.pop_x {
                direction::Horizontal::LeftToRight => (0, rows),
                direction::Horizontal::RightToLeft => (rows, 0),
            };
            AxisData {
                spacing: spacing_x,
                factor: self.pop_x as i8 as i32,
                index: reset,
                reset,
                limiter,
            }
        };
        let mut data_y = {
            let (reset, limiter) = match self.pop_y {
                direction::Vertical::TopToBottom => (0, cols),
                direction::Vertical::BottomToTop => (cols, 0),
            };
            AxisData {
                spacing: spacing_y,
                factor: self.pop_y as i8 as i32,
                index: reset,
                reset,
                limiter,
            }
        };

        // -- Calculate layout --

        let mut children = Vec::with_capacity(self.contents.len());

        for element in &self.contents {
            let child_limits = Limits::new(Size::ZERO, self.cell);

            // Layout child
            let mut layout = element.as_widget().layout(renderer, &child_limits);

            let point_x = data_x.index as f32 * (self.cell.width + data_x.spacing);
            let point_y = data_y.index as f32 * (self.cell.height + data_y.spacing);

            layout.move_to(Point::new(point_x, point_y));
            children.push(layout);

            // Increment accumulators
            match self.order {
                Order::Horizontal => data_x.increment(&mut data_y),
                Order::Vertical => data_y.increment(&mut data_x),
            };
        }

        // Shrink along secondary axis
        let size = match self.order {
            Order::Horizontal => Size {
                height: data_y.index as f32 * (self.cell.height + spacing_y) - spacing_y,
                ..total_size
            },
            Order::Vertical => Size {
                width: data_x.index as f32 * (self.cell.width + spacing_x) - spacing_x,
                ..total_size
            },
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
        self.contents
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
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
{
    #[inline]
    fn from(value: Uniform<'a, Message, Renderer>) -> Self {
        Element::new(value)
    }
}

/// Construct a new [`FlexBox`].
#[inline]
pub fn uniform<'a, Message, Renderer>(
    contents: impl Iterator<Item = Element<'a, Message, Renderer>>,
    cell: Size<u16>,
) -> Uniform<'a, Message, Renderer> {
    Uniform::new(contents, cell)
}
