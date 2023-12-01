//! Row/column widgets that implement re-ordering.

use iced_core::{mouse, Element, Layout, Point, Rectangle, Vector};

pub mod column;
pub use column::Column;

pub mod row;
pub use row::Row;

/// Creates a new [`Column`] with the given children.
#[inline]
pub fn column<Message, Renderer>(
    children: Vec<Element<'_, Message, Renderer>>,
) -> Column<'_, Message, Renderer> {
    Column::with_children(children)
}

/// Creates a new [`Row`] with the given children.
#[inline]
pub fn row<Message, Renderer>(
    children: Vec<Element<'_, Message, Renderer>>,
) -> Row<'_, Message, Renderer> {
    Row::with_children(children)
}

/// Local state.
#[derive(Debug, Clone, Default)]
pub struct State {
    drag_state: Option<Drag>,
}

/// Dragging state of some element.
#[derive(Debug, Clone)]
struct Drag {
    /// Index of the element which is being dragged.
    index: usize,
    /// Point at which dragging begun.
    begun_at: Point,
    /// Current position of the cursor.
    currently_at: Point,
}

impl Drag {
    const ZERO: Self = Self {
        index: usize::MAX,
        begun_at: Point::ORIGIN,
        currently_at: Point::ORIGIN,
    };
}

/// Bind the delta on a child element to be within the bounds of the parent when applied.
#[allow(clippy::collapsible_else_if)]
fn bind_delta(delta: &mut Vector, child: Rectangle, parent: Rectangle) {
    // Bound by x-axis
    if delta.x < 0.0 {
        if child.x + delta.x < parent.x {
            delta.x = parent.x - child.x;
        }
    } else {
        if child.x + child.width + delta.x > parent.x + parent.width {
            delta.x = (parent.x + parent.width) - (child.x + child.width);
        }
    }
    // Bound by y-axis
    if delta.y < 0.0 {
        if child.y + delta.y < parent.y {
            delta.y = parent.y - child.y;
        }
    } else {
        if child.y + child.height + delta.y > parent.y + parent.height {
            delta.y = (parent.y + parent.height) - (child.y + child.height);
        }
    }
}

/// Set the cursor's position only if it is available.
fn set_available_cursor_position(cursor: &mut mouse::Cursor, position: Point) {
    if let mouse::Cursor::Available(_) = cursor {
        *cursor = mouse::Cursor::Available(position)
    }
}

/// Bounds along one axis.
#[derive(Debug, Clone, Copy)]
struct Axis<T: Copy = f32> {
    /// Beginning x/y coordinate.
    coord: T,
    /// Width/Height.
    dimension: T,
}

impl<T: Copy> Axis<T> {
    #[inline]
    const fn x(rect: &Rectangle<T>) -> Self {
        Self {
            coord: rect.x,
            dimension: rect.width,
        }
    }

    #[inline]
    const fn y(rect: &Rectangle<T>) -> Self {
        Self {
            coord: rect.y,
            dimension: rect.height,
        }
    }
}

/// Calculate whether the dragging element should be swapped with an adjacent element.
#[allow(clippy::collapsible_else_if)]
fn should_swap<'a>(
    mut children: impl Iterator<Item = Layout<'a>>,
    drag_state: &Drag,
    drag_coord_key: impl Fn(&Point) -> f32,
    bound_axis: impl Fn(&Rectangle) -> Axis,
) -> Option<(usize, f32)> {
    // Get adjacent elements
    let prev = if drag_state.index > 0 {
        children.nth(drag_state.index - 1)
    } else {
        None
    };
    let curr = children.next().unwrap().bounds();
    let next = children.next();

    // Resolve
    let begun_at = (drag_coord_key)(&drag_state.begun_at);
    let currently_at = (drag_coord_key)(&drag_state.currently_at);

    let curr = (bound_axis)(&curr);

    // Check if there is an overlap with adjacent element and process reorder
    if currently_at > begun_at {
        if let Some(next) = next {
            let next_b = (bound_axis)(&next.bounds());

            if curr.coord + curr.dimension + (currently_at - begun_at)
                > next_b.coord + (next_b.dimension / 2.0)
            {
                Some((drag_state.index + 1, next_b.dimension))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        if let Some(previous) = prev {
            let prev_b = (bound_axis)(&previous.bounds());

            if curr.coord + (currently_at - begun_at) < prev_b.coord + (prev_b.dimension / 2.0) {
                Some((drag_state.index - 1, -prev_b.dimension))
            } else {
                None
            }
        } else {
            None
        }
    }
}
