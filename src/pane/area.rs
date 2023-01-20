//! Pane collection widget.

use iced::widget::pane_grid::{self, Pane as PaneId};
use iced::widget::{button, text, Row};
use iced::{Alignment, Command};

use crate::gui::Element;
use crate::{Pane, Tab};

/// Grid split option.
#[derive(Debug, Clone)]
pub struct Split {
    /// Axis along which the new split must occur.
    axis: pane_grid::Axis,
    /// Id of the pane to split.
    ///
    /// If [`None`], is the focused pane.
    pane: Option<PaneId>,
}

/// Messages from the controller section of each pane.
#[derive(Debug, Clone)]
pub enum ControlMessage {
    Split(Split),
    Maximize(PaneId),
    Restore,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// A message to a single pane.
    Pane(super::Message, Option<PaneId>),
    Control(ControlMessage),
    // Grid messages
    // These and their handlers are more or less copied exactly from iced examples
    // https://github.com/iced-rs/iced/blob/master/examples/pane_grid/src/main.rs
    Clicked(PaneId),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
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

    /// Get the currently focused tab of the currently focused pane.
    #[inline]
    pub fn focused(&self) -> &Pane {
        self.panes.get(&self.focused.unwrap()).unwrap()
    }

    pub fn update(&mut self, message: Message) -> anyhow::Result<Command<Message>> {
        let mut commands = vec![];

        match message {
            Message::Pane(m, id_or) => {
                if let Some(id) = id_or.map_or(self.focused, Some) {
                    if let Some(pane) = self.panes.get_mut(&id) {
                        let pane_cmd = pane.update(m)?;
                        commands.push(pane_cmd.map(move |m| Message::Pane(m, id_or)))
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
            Message::Control(p) => match p {
                ControlMessage::Maximize(pane) => self.panes.maximize(&pane),
                ControlMessage::Restore => self.panes.restore(),
                ControlMessage::Split(split) => {
                    if let Some(pane) = split.pane.map_or(self.focused, Some) {
                        let result = self.panes.split(split.axis, &pane, Pane::new(Tab::new()?));

                        if let Some((pane, _)) = result {
                            self.focused = Some(pane);
                        }
                    }
                }
            },
            _ => {}
        }

        Ok(Command::batch(commands))
    }

    pub fn view(&self) -> Element<Message> {
        // Controller list
        fn view_controls(controls: Vec<Element<ControlMessage>>) -> Element<Message> {
            let mut control_list = Row::new()
                .align_items(Alignment::Center)
                .padding(6)
                .spacing(4);

            for control in controls {
                control_list = control_list.push(control.map(Message::Control));
            }

            control_list.into()
        }

        let grid = pane_grid::PaneGrid::new(&self.panes, |id, state, _focused| {
            let top_bar = pane_grid::TitleBar::new(
                state
                    .top_bar()
                    .unwrap()
                    .map(move |m| Message::Pane(m, Some(id))),
            )
            .controls(view_controls(vec![button(text("split"))
                .on_press(ControlMessage::Split(Split {
                    axis: pane_grid::Axis::Horizontal,
                    pane: Some(id),
                }))
                .into()]));

            pane_grid::Content::new(state.view().map(move |m| Message::Pane(m, Some(id))))
                .title_bar(top_bar)
        })
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        grid.into()
    }
}
