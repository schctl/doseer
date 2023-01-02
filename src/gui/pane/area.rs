//! Pane collection widget.

use iced::widget::pane_grid::{self, Pane as PaneId};

use super::{tab, Pane, Tab};
use crate::gui::Theme;

#[derive(Debug, Clone)]
pub enum Message {
    /// A message to a single pane.
    Pane(super::Message, PaneId),
    // Grid messages
    // These and their handlers are more or less copied exactly from iced examples
    // https://github.com/iced-rs/iced/blob/master/examples/pane_grid/src/main.rs
    Split(pane_grid::Axis, PaneId),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(PaneId),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Maximize(PaneId),
    Restore,
    Close(PaneId),
    CloseFocused,
}

/// The main space where all panes are displayed.
pub struct Area {
    /// Panegrid internal state.
    panes: pane_grid::State<Pane>,
    /// Focused pane.
    focused: Option<PaneId>,
}

impl Area {
    pub fn new(pane: Pane) -> Self {
        let (panes, id) = pane_grid::State::new(pane);

        Self {
            panes,
            focused: Some(id),
        }
    }

    pub fn update(&mut self, message: Message) -> anyhow::Result<()> {
        match message {
            Message::Pane(m, id) => {
                let pane = self.panes.get_mut(&id).unwrap();
                pane.update(m)?;
            }
            Message::Split(axis, pane) => {
                let result = self.panes.split(axis, &pane, Pane::new(Tab::new()?));

                if let Some((pane, _)) = result {
                    self.focused = Some(pane);
                }
            }
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.focused {
                    let result = self.panes.split(axis, &pane, Pane::new(Tab::new()?));

                    if let Some((pane, _)) = result {
                        self.focused = Some(pane);
                    }
                }
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.focused {
                    if let Some(adjacent) = self.panes.adjacent(&pane, direction) {
                        self.focused = Some(adjacent);
                    }
                }
            }
            Message::Clicked(pane) => {
                self.focused = Some(pane);
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(&split, ratio);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.swap(&pane, &target);
            }
            Message::Maximize(pane) => self.panes.maximize(&pane),
            Message::Restore => {
                self.panes.restore();
            }
            Message::Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(&pane) {
                    self.focused = Some(sibling);
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.focused {
                    if let Some((_, sibling)) = self.panes.close(&pane) {
                        self.focused = Some(sibling);
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn view(&self) -> anyhow::Result<iced::Element<'_, Message, iced::Renderer<Theme>>> {
        let grid = pane_grid::PaneGrid::new(&self.panes, |id, state, _focused| {
            pane_grid::Content::new(
                state
                    .view(super::ViewOpts {
                        tab: tab::ViewOpts { columns: 6 },
                    })
                    .unwrap()
                    .map(move |m| Message::Pane(m, id)),
            )
        })
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        Ok(grid.into())
    }
}
