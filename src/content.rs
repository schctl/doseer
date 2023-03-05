//! Main content area.

use doseer_core::path::PathWrap;
use doseer_ui_ext::widgets::only_one;
use doseer_ui_ext::widgets::reorderable;

use iced::widget::{button, column, container, row, text};
use iced::{alignment, Alignment, Color, Command, Length};
use iced_lazy::component;
use indexmap::IndexMap;
use sleet::ColorScheme;

use crate::gui::Element;
use crate::tab::tab;
use crate::{tab, theme, Icon, Tab, Theme};

/// Main content. Essentially just manages tabs.
#[derive(Debug)]
pub struct Content {
    /// Tabs held by this pane.
    tabs: IndexMap<usize, tab::State>,
    /// Currently open tab.
    focused: usize,
}

impl Content {
    pub fn new() -> Self {
        let mut tabs = IndexMap::new();
        tabs.insert(0, tab::State::new().expect("failed to create tab"));

        Self { tabs, focused: 0 }
    }

    /// Add a new tab to this pane.
    #[inline]
    pub fn add_tab(&mut self, tab: tab::State) -> usize {
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
    pub fn replace_focused(&mut self, tab: tab::State) {
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
    pub fn focused(&self) -> &tab::State {
        self.tabs.get(&self.focused).unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    /// A message to a single tab.
    Tab(tab::Message, Option<usize>),
    /// Focus the indicated tab.
    Focus(usize),
    /// Remove the indicated tab.
    Remove(usize),
    /// Add a new tab, and maybe focus.
    New(Option<PathWrap>, bool),
    /// Replace the focused tab with a new tab.
    Replace(PathWrap),
    /// Reorder two tab positions.
    Reorder(usize, usize),
}

impl Content {
    pub fn update(&mut self, message: Message) -> anyhow::Result<Command<Message>> {
        let mut commands = vec![];

        match message {
            Message::Tab(m, id) => match m {
                tab::Message::Open(p) => {
                    let watcher = self
                        .tabs
                        .get_mut(&id.unwrap_or(self.focused))
                        .unwrap()
                        .open(&p)?;

                    commands.push(watcher.map(move |m| Message::Tab(m, id)));
                }
                _ => {}
            },
            Message::Focus(id) => {
                self.focus(id);
            }
            Message::Remove(id) => {
                self.remove_tab(id);
            }
            Message::New(path, focus) => {
                let tab = match path {
                    Some(t) => tab::State::new_with(t)?,
                    None => tab::State::new()?,
                };

                let id = self.add_tab(tab);

                if focus {
                    self.focus(id);
                }
            }
            Message::Replace(tab) => self.replace_focused(tab::State::new_with(tab)?),
            Message::Reorder(a, b) => {
                self.tabs.swap_indices(a, b);
            }
        }

        Ok(Command::batch(commands))
    }

    pub const TOP_BAR_HEIGHT: Length = Length::Fixed(50.0);

    /// Tab switcher and controls.
    fn tab_controls(&self) -> Element<Message> {
        let mut tab_list = reorderable::Row::new()
            .align_items(Alignment::Center)
            .spacing(4)
            .height(Length::Fill)
            .on_reorder(Message::Reorder);

        // --- Tab Buttons ---

        for (index, tab) in &self.tabs {
            let folder_name = row!(
                Icon::Directory
                    .svg()
                    .width(Length::Fixed(22.0))
                    .height(Length::Fixed(22.0)),
                text(tab.location().display().to_string_lossy())
                    .size(22)
                    .font(theme::fonts::UI::Regular),
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
                        .height(Length::Fixed(18.0))
                        .width(Length::Fixed(18.0))
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
                .height(Length::Fixed(24.0))
                .width(Length::Fixed(24.0)),
            )
            .style(TabButtonStyle::Default.into())
            .on_press(Message::Remove(*index));

            // create tab as a button
            let contents = row!(folder_name.width(Length::Fixed(186.0)), close_button);

            let tab = button(contents)
                // focus tab when the button is pressed
                .on_press(Message::Focus(*index))
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

        // --- Construct Top Bar ---

        let mut elements = row!(tab_list)
            .align_items(Alignment::Center)
            .padding(6)
            .spacing(4)
            .height(Self::TOP_BAR_HEIGHT);

        elements = elements.push(
            button(
                container(
                    Icon::Plus
                        .svg()
                        .height(Length::Fixed(18.0))
                        .width(Length::Fixed(18.0))
                        .style(theme::svg::Neutral::Bright1.into()),
                )
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .height(Length::Fixed(24.0))
                .width(Length::Fixed(24.0)),
            )
            .style(TabButtonStyle::Default.into())
            .on_press(Message::New(None, true)),
        );

        elements.into()
    }

    pub fn view(&self) -> Element<Message> {
        // Tab switcher
        let panel = self.tab_controls();

        // Focused tab view
        let contents = only_one(
            self.tabs
                .values()
                .map(|t| component(tab(t)).map(move |m| Message::Tab(m, None))),
        )
        // We need to get the _index_ of the focused tab
        .focus(self.tabs.keys().position(|k| *k == self.focused).unwrap());

        // TODO: define panel position at runtime
        column!(panel, contents).into()
    }
}

/// Tab selector button theme.
#[derive(Debug, Clone, Default)]
pub enum TabButtonStyle {
    #[default]
    Default,
    Focused,
}

impl From<TabButtonStyle> for theme::button::Button {
    fn from(t: TabButtonStyle) -> Self {
        Self::Tab(t)
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
