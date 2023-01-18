//! Pane widget.

use std::convert::identity;

use m7_core::path::PathWrap;

use iced::widget::{button, container, row, text, Row};
use iced::{alignment, Alignment, Color, Length};
use indexmap::IndexMap;
use sleet::style::ColorScheme;

use crate::gui::Element;
use crate::{tab, theme, Icon, Tab, Theme};

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
}

pub struct ViewOpts {
    pub tab: tab::ViewOpts,
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

    pub const TOP_BAR_HEIGHT: Length = Length::Units(50);

    pub fn top_bar<'a>(&'a self) -> anyhow::Result<Element<'a, Message>> {
        // Pane area provided controllers

        // Held tab list
        let mut tab_list = Row::new()
            .align_items(Alignment::Center)
            .padding(6)
            .spacing(4)
            .height(Self::TOP_BAR_HEIGHT);

        for (index, tab) in &self.tabs {
            // the name and icon of the tab
            let folder_name = row!(
                Icon::Directory
                    .svg()
                    .width(Length::Units(22))
                    .height(Length::Units(22)),
                text(tab.location().display().to_string_lossy()).size(22),
            )
            .spacing(6)
            .align_items(Alignment::Center)
            .height(Length::Fill)
            .padding([0, 4]);

            // close this tab
            let close_button = button(
                container(
                    Icon::Cross
                        .svg()
                        .height(Length::Units(18))
                        .width(Length::Units(18))
                        .style(
                            if *index == self.focused {
                                theme::svg::Neutral::Bright1
                            } else {
                                theme::svg::Neutral::Bright0
                            }
                            .into(),
                        ),
                )
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .height(Length::Units(24))
                .width(Length::Units(24)),
            )
            .style(TabButtonStyle::Default.into())
            .on_press(Message::Tab(TabMessage::Remove, Some(*index)));

            // create tab as a button
            let contents = row!(folder_name.width(Length::Units(186)), close_button);

            let tab = button(contents)
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
            container(
                Icon::Plus
                    .svg()
                    .height(Length::Units(18))
                    .width(Length::Units(18))
                    .style(theme::svg::Neutral::Bright1.into()),
            )
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .height(Length::Units(24))
            .width(Length::Units(24)),
        )
        .style(TabButtonStyle::Default.into())
        .on_press(Message::Tab(TabMessage::New, None));

        tab_list = tab_list.push(new_tab);

        Ok(tab_list.into())
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
