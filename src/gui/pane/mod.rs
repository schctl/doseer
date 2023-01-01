//! Pane widget.

use iced::widget::svg::Handle;
use iced::widget::{button, row, text, Row, Svg};
use iced::{Alignment, Length, Padding};
use indexmap::IndexMap;

use crate::gui::Theme;

pub mod tab;
use tab::Tab;

use super::theme;

pub mod icons;
pub mod item;

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

    /// Get all held tabs.
    #[inline]
    pub fn tabs(&self) -> &IndexMap<usize, Tab> {
        &self.tabs
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
}

#[derive(Debug, Clone)]
pub enum Message {
    Tab(TabMessage, usize),
}

pub struct ViewOpts {
    pub tab: tab::ViewOpts,
}

impl Pane {
    pub fn update(&mut self, message: Message) -> anyhow::Result<()> {
        match message {
            Message::Tab(message, index) => match message {
                TabMessage::Internal(i) => {
                    self.tabs.get_mut(&index).unwrap().update(i)?;
                }
                TabMessage::Focus => {
                    self.focus(index);
                }
                TabMessage::Remove => {
                    self.remove_tab(index);
                }
            },
        }

        Ok(())
    }

    pub fn view(
        &self,
        opts: ViewOpts,
    ) -> anyhow::Result<iced::Element<'_, Message, iced::Renderer<Theme>>> {
        // Create top tab view
        let mut tab_list = Row::new()
            .align_items(Alignment::Center)
            .padding(6)
            .spacing(4);

        for (index, tab) in &self.tabs {
            // create tab as a button
            let tab = button(
                row!(
                    Svg::new(Handle::from_memory(icons::DIRECTORY))
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
                .padding(Padding::from([0, 4])),
            )
            // focus tab when the button is pressed
            .on_press(Message::Tab(TabMessage::Focus, *index))
            .style(
                if *index == self.focused {
                    TabTheme::Focused
                } else {
                    TabTheme::Unfocused
                }
                .into(),
            );

            tab_list = tab_list.push(tab);
        }

        // Focused tab view
        let focused = self.tabs.get(&self.focused).unwrap();

        let view = focused
            .view(opts.tab)?
            .map(|i| Message::Tab(TabMessage::Internal(i), self.focused));

        // Final view
        let final_view = iced::widget::column!(tab_list, view);

        Ok(final_view.into())
    }
}

/// Tab button theme.
#[derive(Debug, Clone, Default)]
pub enum TabTheme {
    Focused,
    #[default]
    Unfocused,
}

impl From<TabTheme> for theme::Button {
    fn from(t: TabTheme) -> Self {
        theme::Button::Tab(t)
    }
}

impl TabTheme {
    pub fn active(&self, theme: &Theme) -> iced::widget::button::Appearance {
        let base = theme.base();
        let normal = theme.normal();

        match self {
            Self::Focused => iced::widget::button::Appearance {
                background: normal.bg.into(),
                text_color: base.fg,
                border_radius: 6.0,
                ..Default::default()
            },
            Self::Unfocused => iced::widget::button::Appearance {
                background: base.bg.into(),
                text_color: base.fg,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }

    pub fn hovered(&self, theme: &Theme) -> iced::widget::button::Appearance {
        let base = theme.base();
        let normal = theme.normal();

        match self {
            Self::Focused => self.active(theme),
            Self::Unfocused => iced::widget::button::Appearance {
                background: normal.bg.into(),
                text_color: base.fg,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }
}
