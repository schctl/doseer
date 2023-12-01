//! Grid widgets.

pub mod flexbox;
pub use flexbox::{flexbox, FlexBox};

pub mod uniform;
pub use uniform::{uniform, Uniform};

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
