//! Pane widget.

use std::convert::identity;

use iced::widget::{button, row, text, Row};
use iced::{Alignment, Color, Length};
use indexmap::IndexMap;
use sleet::style::ColorScheme;

use crate::gui::{icons::Icon, tab, theme, Element, Tab, Theme};
use crate::path::PathWrap;

pub mod area;
pub use area::Area;

/// A pane contains many tabs, but displays only one at a time.
#[derive(Debug)]
pub struct Pane {
    /// Tabs held by this pane.
    tabs: IndexMap<usize, Tab>,
    /// Currently open tab.
    focused: usize,
}

impl Pane {
    pub fn new(tab: Tab) -> Self {
        let mut tabs = IndexMap::new();
        tabs.insert(0, tab);

        Self { tabs, focused: 0 }
    }

    /// Add a new tab to this pane.
    #[inline]
    pub fn add_tab(&mut self, tab: Tab) -> usize {
        let index = self.tabs.last().unwrap().0 + 1;
        self.tabs.insert(index, tab);
        index
    }

    /// Remove a tab from this pane.
    pub fn remove_tab(&mut self, tab: usize) -> Option<Tab> {
        // Only remove tab if its not the only one remaining.
        if self.tabs.len() > 1 {
            self.tabs.remove(&tab)?;

            // Change focus
            if self.focused == tab {
                self.focused = *self.tabs.last().unwrap().0;
            }
        }

        None
    }

    /// Replace the currently focused tab with another tab.
    pub fn replace_focused(&mut self, tab: Tab) {
        self.tabs.remove(&self.focused);
        self.tabs.insert(self.focused, tab);
    }

    /// Bring focus to a tab.
    pub fn focus(&mut self, tab: usize) -> Option<()> {
        if self.tabs.get(&tab).is_some() {
            self.focused = tab;
            return Some(());
        }

        None
    }

    /// Get currently focused tab.
    #[inline]
    pub fn focused(&self) -> &Tab {
        self.tabs.get(&self.focused).unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum TabMessage {
    Internal(tab::Message),
    Focus,
    Remove,
    New,
    Replace(PathWrap),
}

#[derive(Debug, Clone)]
pub enum Message {
    Tab(TabMessage, Option<usize>),
    /// For external use by the pane area.
    Control(area::ControlMessage),
}

pub struct ViewOpts {
    pub tab: tab::ViewOpts,
}

pub struct TopBarOpts<'a> {
    pub controls: Vec<Element<'a, area::ControlMessage>>,
}

impl Pane {
    pub fn update(&mut self, message: Message) -> anyhow::Result<()> {
        match message {
            Message::Tab(message, index) => match message {
                TabMessage::Internal(i) => {
                    self.tabs
                        .get_mut(&index.map_or(self.focused, identity))
                        .unwrap()
                        .update(i)?;
                }
                TabMessage::Focus => {
                    self.focus(index.map_or(self.focused, identity));
                }
                TabMessage::Remove => {
                    self.remove_tab(index.map_or(self.focused, identity));
                }
                TabMessage::New => {
                    let index = self.add_tab(Tab::new()?);
                    self.focus(index);
                }
                TabMessage::Replace(tab) => self.replace_focused(Tab::new_with(tab)?),
            },
            _ => {
                tracing::error!("invalid message received: {:?}", message);
            }
        }

        Ok(())
    }

    pub const TOP_BAR_HEIGHT: Length = Length::Units(52);

    pub fn top_bar<'a>(&'a self, opts: TopBarOpts<'a>) -> anyhow::Result<Element<'a, Message>> {
        // Pane area provided controllers
        let mut control_list = Row::new()
            .align_items(Alignment::Center)
            .padding(6)
            .height(Length::Fill)
            .spacing(4);

        for control in opts.controls {
            control_list = control_list.push(control.map(Message::Control));
        }

        // Held tab list
        let mut tab_list = Row::new()
            .align_items(Alignment::Center)
            .padding(6)
            .spacing(4)
            .height(Length::Fill)
            .width(Length::Fill);

        for (index, tab) in &self.tabs {
            // create tab as a button
            let tab = button(
                row!(
                    Icon::Directory
                        .svg()
                        .width(Length::Units(22))
                        .height(Length::Units(22)),
                    text(
                        // get file name location
                        tab.location()
                            .canonicalize()?
                            .file_name()
                            // unwrap ok since name is canonicalized
                            .unwrap()
                            .to_string_lossy(),
                    )
                    .size(22),
                )
                .spacing(6)
                .align_items(Alignment::Center)
                .width(Length::Units(186))
                .height(Length::Units(28))
                .padding([0, 4]),
            )
            // focus tab when the button is pressed
            .on_press(Message::Tab(TabMessage::Focus, Some(*index)))
            .style(
                if *index == self.focused {
                    TabButtonStyle::Focused
                } else {
                    TabButtonStyle::Default
                }
                .into(),
            );

            tab_list = tab_list.push(tab);
        }

        let new_tab = button(
            Icon::Plus
                .svg()
                .height(Length::Units(26))
                .width(Length::Units(26)),
        )
        .style(TabButtonStyle::Default.into())
        .on_press(Message::Tab(TabMessage::New, None));

        tab_list = tab_list.push(new_tab);

        Ok(row!(tab_list, control_list)
            .height(Self::TOP_BAR_HEIGHT)
            .into())
    }

    pub fn view(&self, opts: ViewOpts) -> Element<Message> {
        // Focused tab view
        let view = self
            .focused()
            .view(opts.tab)
            .map(|i| Message::Tab(TabMessage::Internal(i), None));

        view
    }
}

/// Tab button theme.
#[derive(Debug, Clone, Default)]
pub enum TabButtonStyle {
    #[default]
    Default,
    Focused,
}

impl From<TabButtonStyle> for theme::Button {
    fn from(t: TabButtonStyle) -> Self {
        theme::Button::Tab(t)
    }
}

impl TabButtonStyle {
    pub fn active(&self, theme: &Theme) -> iced::widget::button::Appearance {
        let palette = theme.palette();

        match self {
            Self::Focused => iced::widget::button::Appearance {
                background: palette.surface.base.base.into(),
                text_color: palette.surface.base.on_base,
                border_radius: 6.0,
                ..Default::default()
            },
            Self::Default => iced::widget::button::Appearance {
                background: Color::TRANSPARENT.into(),
                text_color: palette.surface.weak.on_base,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }

    pub fn hovered(&self, theme: &Theme) -> iced::widget::button::Appearance {
        let palette = theme.palette();

        match self {
            Self::Focused => self.active(theme),
            Self::Default => iced::widget::button::Appearance {
                background: palette.surface.base.base.into(),
                text_color: palette.surface.base.on_base,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }

    pub fn pressed(&self, theme: &Theme) -> iced::widget::button::Appearance {
        Self::Focused.active(theme)
    }
}
