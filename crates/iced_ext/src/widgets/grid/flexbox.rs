//! Grid with a flexbox layout.

use iced_core::widget::{Operation, Tree};
use iced_core::{
    event, layout, mouse, Clipboard, Element, Event, Length, Point, Rectangle, Shell, Size, Widget,
};
use iced_core::{overlay, renderer};
pub use iced_widget::responsive;

use super::{direction, Order};

/// A dynamic linearly-populated grid with a flexbox layout.
///
/// Grids have a defined population order and direction, so they can be populated linearly. These
/// grids also have no defined cell size. Intended to be used as a [`responsive`] widget.
///
/// ## Warning!
///
/// Even though this widget is called "FlexBox", it implements a jank home-grown version of the algorithm
/// so don't expect it do exactly what flexbox does. It is just intended to be usable in the main layout
/// of `doseer` and a few smaller areas.
#[must_use]
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

    /// X-axis spacing.
    spacing_x: f32,
    /// Y-axis spacing.
    spacing_y: f32,

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
            spacing_x: 0.0,
            spacing_y: 0.0,
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
        state.diff_children(&self.contents);
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        // -- Accumulators --

        struct AxisData {
            /// Spacing along this axis
            spacing: f32,
            /// Bounds increment factor
            factor: f32,
            /// Current bounds along this axis.
            accum: f32,
            /// Starting bounds.
            reset: f32,
            /// Max bounds.
            limiter: f32,
        }

        let data_x = {
            let (reset, limiter) = match self.pop_x {
                direction::Horizontal::LeftToRight => (0.0, limits.max().width),
                direction::Horizontal::RightToLeft => (limits.max().width, 0.0),
            };
            AxisData {
                spacing: self.spacing_x,
                factor: self.pop_x as i8 as f32,
                accum: reset,
                reset,
                limiter,
            }
        };
        let data_y = {
            let (reset, limiter) = match self.pop_y {
                direction::Vertical::TopToBottom => (0.0, limits.max().height),
                direction::Vertical::BottomToTop => (limits.max().height, 0.0),
            };
            AxisData {
                spacing: self.spacing_y,
                factor: self.pop_y as i8 as f32,
                accum: reset,
                reset,
                limiter,
            }
        };

        // One more accumulator so we can get the maximum breadth of this section on the cross axis
        let mut next_accum = match self.order {
            Order::Horizontal => data_x.accum,
            Order::Vertical => data_y.accum,
        };

        // -- Resolve main and cross axes --

        let (mut main_axis, mut cross_axis) = match self.order {
            Order::Horizontal => (data_x, data_y),
            Order::Vertical => (data_y, data_x),
        };
        // Resolve (main, cross) into (x, y)
        let resolve = match self.order {
            Order::Horizontal => |main, cross| (main, cross),
            Order::Vertical => |main, cross| (cross, main),
        };
        // Resolve (x, y) into (main, cross)
        let revert = match self.order {
            Order::Horizontal => |x, y| (x, y),
            Order::Vertical => |x, y| (y, x),
        };
        // Shrink the cross axis by `delta` units
        let shrink_cross_axis = match self.order {
            Order::Horizontal => |limits: layout::Limits, delta: f32| {
                limits.max_height((limits.max().height - delta).max(0.0))
            },
            Order::Vertical => |limits: layout::Limits, delta: f32| {
                limits.max_width((limits.max().width - delta).max(0.0))
            },
        };

        // -- Calculate layout --

        // TODO: handle `Length::Fill` and `Length::FilePortion` on children

        let naive_insert = |layout: &mut layout::Node,
                            main_axis: &mut AxisData,
                            cross_axis: &AxisData,
                            next_accum: &mut f32| {
            let resolved = (resolve)(main_axis.accum, cross_axis.accum);
            layout.move_to(Point::new(resolved.0, resolved.1));

            let bounds = layout.bounds();
            let size_along = (revert)(bounds.width, bounds.height);
            let position_along = (revert)(bounds.x, bounds.y);

            // Accumulate along our main axis
            main_axis.accum += (size_along.0 + main_axis.spacing) * main_axis.factor;
            // Accumulate along cross axis only if it extends it
            if size_along.1 + position_along.1 + cross_axis.spacing > *next_accum {
                *next_accum += (size_along.1 + cross_axis.spacing) + cross_axis.factor;
            }
        };

        let mut children = Vec::with_capacity(self.contents.len());

        for element in &self.contents {
            let mut layout = element.as_widget().layout(renderer, &limits);
            let bounds = layout.bounds();
            let size_along = (revert)(bounds.width, bounds.height);

            // Try to insert into current row/column
            if size_along.0 + main_axis.accum < main_axis.limiter {
                // Easy!
                if size_along.1 + cross_axis.accum < cross_axis.limiter {
                    (naive_insert)(&mut layout, &mut main_axis, &cross_axis, &mut next_accum);
                }
                // Our element extends the cross axis limits.
                // So shrink them and recalculate layout.
                else {
                    let mut shrunk_limits = limits;
                    let delta = (size_along.1 + cross_axis.accum) - cross_axis.limiter;
                    shrunk_limits = (shrink_cross_axis)(shrunk_limits, delta);

                    layout = element.as_widget().layout(renderer, &shrunk_limits);
                    let resolved = (resolve)(main_axis.accum, cross_axis.accum);
                    layout.move_to(Point::new(resolved.0, resolved.1));
                    next_accum = cross_axis.limiter;
                }
            // Move to next row/column
            } else {
                // Extended cross axis. Stop here.
                if next_accum >= cross_axis.limiter {
                    break;
                }
                // Reset
                main_axis.accum = main_axis.reset;
                cross_axis.accum = next_accum;
                (naive_insert)(&mut layout, &mut main_axis, &cross_axis, &mut next_accum);
            }

            children.push(layout);
        }

        let resolved = (resolve)(main_axis.accum, next_accum);
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
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((child, state), layout) in self
            .contents
            .iter()
            .zip(&state.children)
            .zip(layout.children())
        {
            child
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
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
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
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
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.contents
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

    fn overlay<'overlay>(
        &'overlay mut self,
        state: &'overlay mut Tree,
        layout: iced_core::Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_core::overlay::Element<'overlay, Message, Renderer>> {
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

/// Construct a new [`FlexBox`] grid.
#[inline]
pub fn flexbox<'a, Message, Renderer>(
    contents: impl Iterator<Item = Element<'a, Message, Renderer>>,
) -> FlexBox<'a, Message, Renderer> {
    FlexBox::new(contents)
}
