//! Grid with a flexbox layout.

pub use iced_lazy::responsive;
use iced_native::widget::{Operation, Tree};
use iced_native::{
    event, layout, mouse, Clipboard, Element, Event, Length, Point, Rectangle, Shell, Size, Widget,
};
use iced_native::{overlay, renderer};

/// Defines the direction of population.
pub mod direction {
    /// Population order along the x-axis.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Horizontal {
        /// Start populating from the left side of the grid.
        #[default]
        LeftToRight = 1,
        /// Start populating from the right side of the grid.
        RightToLeft = -1,
    }

    /// Population order along the y-axis.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Vertical {
        /// Start populating from the top of the grid.
        #[default]
        TopToBottom = 1,
        /// Start populating from the bottom of the grid.
        BottomToTop = -1,
    }
}

/// Overall grid population order.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Order {
    /// Populate along the x-axis first.
    #[default]
    Horizontal,
    /// Populate along the y-axis first.
    Vertical,
}

/// A dynamic linearly-populated grid with a flexbox layout.
///
/// Grids have a defined population order and direction, so they can be populated linearly. These
/// grids also have no defined cell size. Intended to be used as a [`responsive`] widget.
pub struct FlexBox<'a, Message, Renderer> {
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
    /// Limits to be applied on elements.

    /// X-axis spacing.
    spacing_x: u16,
    /// Y-axis spacing.
    spacing_y: u16,

    /// Contents of the grid.
    contents: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> FlexBox<'a, Message, Renderer> {
    /// Construct a new linear grid with the given contents.
    ///
    /// The grid will be populated will elements in the same order as the given iterator.
    #[inline]
    pub fn new(contents: impl Iterator<Item = Element<'a, Message, Renderer>>) -> Self {
        Self {
            pop_x: Default::default(),
            pop_y: Default::default(),
            order: Default::default(),
            width: Length::Fill,
            height: Length::Fill,
            spacing_x: 0,
            spacing_y: 0,
            contents: contents.collect(),
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
        self.spacing_x = units;
        self
    }

    /// Sets the vertical spacing _between_ the cells of the grid.
    pub fn spacing_y(mut self, units: u16) -> Self {
        self.spacing_y = units;
        self
    }
}

impl<'a, Renderer, Message> Widget<Message, Renderer> for FlexBox<'a, Message, Renderer>
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
        state.diff_children(&self.contents)
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        // -- Accumulators --

        struct AxisData {
            max_size: f32,
            spacing: f32,
            factor: f32,
            accum: f32,
        }

        let x_data = AxisData {
            max_size: limits.max().width,
            spacing: self.spacing_x as f32,
            factor: self.pop_x as i8 as f32,
            accum: 0.0,
        };
        let y_data = AxisData {
            max_size: limits.max().height,
            spacing: self.spacing_y as f32,
            factor: self.pop_y as i8 as f32,
            accum: 0.0,
        };
        let mut next_accum = 0.0;

        // -- Resolve first and second axes --

        let (mut first_axis, mut second_axis) = match self.order {
            Order::Horizontal => (x_data, y_data),
            Order::Vertical => (y_data, x_data),
        };

        let resolve = |first_axis_point, second_axis_point| match self.order {
            Order::Horizontal => (first_axis_point, second_axis_point),
            Order::Vertical => (second_axis_point, first_axis_point),
        };
        let revert = |x, y| match self.order {
            Order::Horizontal => (x, y),
            Order::Vertical => (y, x),
        };
        let shrink_second_axis = |limits: layout::Limits, delta: f32| match self.order {
            Order::Horizontal => limits.max_height((limits.max().height - delta).max(0.0) as u32),
            Order::Vertical => limits.max_width((limits.max().width - delta).max(0.0) as u32),
        };

        // -- Calculate layout --

        let naive_insert = |layout: &mut layout::Node,
                            first_axis: &mut AxisData,
                            second_axis: &AxisData,
                            next_accum: &mut f32| {
            let resolved = (resolve)(first_axis.accum, second_axis.accum);
            layout.move_to(Point::new(resolved.0, resolved.1));

            let bounds = layout.bounds();
            let size_along = (revert)(bounds.width, bounds.height);
            let position_along = (revert)(bounds.x, bounds.y);

            // Accumulate along our first axis
            first_axis.accum += (size_along.0 + first_axis.spacing) * first_axis.factor;
            // Accumulate along second axis only if it extends it
            if size_along.1 + second_axis.spacing + position_along.1 > *next_accum {
                *next_accum += (size_along.1 + second_axis.spacing) + second_axis.factor;
            }
        };

        let mut children = Vec::new();

        for element in &self.contents {
            let mut layout = element.as_widget().layout(renderer, &limits);
            let bounds = layout.bounds();
            let size_along = (revert)(bounds.width, bounds.height);

            // Try to insert into current row/column
            if size_along.0 + first_axis.accum < first_axis.max_size {
                // Easy!
                if size_along.1 + second_axis.accum < second_axis.max_size {
                    (naive_insert)(&mut layout, &mut first_axis, &second_axis, &mut next_accum);
                }
                // Our element extends the second axis.
                // So shrink the limits so it doesn't, and recalculate layout.
                else {
                    let mut shrunk_limits = limits;
                    let delta = (size_along.1 + second_axis.accum) - second_axis.max_size;
                    shrunk_limits = (shrink_second_axis)(shrunk_limits, delta);

                    layout = element.as_widget().layout(renderer, &shrunk_limits);
                    let resolved = (resolve)(first_axis.accum, second_axis.accum);
                    layout.move_to(Point::new(resolved.0, resolved.1));
                    next_accum = second_axis.max_size;
                }
            // Move to next row/column
            } else {
                // Extended second axis. Stop here.
                if next_accum > second_axis.max_size {
                    break;
                } else {
                    // Reset
                    first_axis.accum = 0.0;
                    second_axis.accum = next_accum;
                    (naive_insert)(&mut layout, &mut first_axis, &second_axis, &mut next_accum);
                }
            }

            children.push(layout);
        }

        let resolved = (resolve)(first_axis.accum, next_accum);
        let size = Size::new(resolved.0, resolved.1);

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
                })
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

impl<'a, Message, Renderer> From<FlexBox<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
{
    #[inline]
    fn from(value: FlexBox<'a, Message, Renderer>) -> Self {
        Element::new(value)
    }
}

/// Construct a new [`FlexBox`].
#[inline]
pub fn flexbox<'a, Message, Renderer>(
    contents: impl Iterator<Item = Element<'a, Message, Renderer>>,
) -> FlexBox<'a, Message, Renderer> {
    FlexBox::new(contents)
}