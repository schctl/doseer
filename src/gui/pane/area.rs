//! Pane collection widget.

use iced::widget::pane_grid::{self, Pane as PaneId};

use super::tab;
use super::Pane;
use crate::gui::Theme;

#[derive(Debug, Clone)]
pub enum Message {
    /// A message to a single pane.
    Pane(super::Message, PaneId),
}

/// The main space where all panes are displayed.
pub struct Area {
    /// Panegrid internal state.
    panes: pane_grid::State<Pane>,
    /// Focused pane.
    focused: PaneId,
}

impl Area {
    pub fn new(pane: Pane) -> Self {
        let (panes, focused) = pane_grid::State::new(pane);

        Self { panes, focused }
    }

    pub fn update(&mut self, message: Message) -> anyhow::Result<()> {
        match message {
            Message::Pane(m, id) => {
                let pane = self.panes.get_mut(&id).unwrap();
                pane.update(m)
            }
        }
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
        });

        Ok(grid.into())
    }
}
