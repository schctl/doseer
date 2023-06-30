//! Stopgap solution to make prototyping custom themes for `iced` quicker.
//!
//! This module provides the `DevAuto` trait for all built-in iced widgets which can implement a "default"
//! stylesheet for that widget. This helps reduce some boilerplate and lets you put-off writing some
//! stylesheets for later. Eventually, all `DevAuto` impls should be replaced by your custom theme.

use derive_more::{Deref, DerefMut, From};

pub mod application;
pub mod button;
pub mod container;
pub mod pane_grid;
pub mod rule;
pub mod scrollable;
pub mod svg;
pub mod text;

pub use application::Application;
pub use button::Button;
pub use container::Container;
pub use pane_grid::PaneGrid;
pub use rule::Rule;
pub use scrollable::Scrollable;
pub use svg::Svg;
pub use text::Text;

/// A wrapper type which exists so stylesheets can be auto-implemented.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, From, Deref, DerefMut, Default)]
pub struct Wrap<T>(pub T);
