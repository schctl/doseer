//! Row/column widgets that implement re-ordering.

use iced_native::{Point, Rectangle, Vector};

// TODO: column

pub mod row;
pub use row::Row;

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

/// Local state.
#[derive(Debug, Clone, Default)]
pub struct State {
    drag_state: Option<Drag>,
}
