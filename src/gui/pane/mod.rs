//! Pane widget.

use iced::widget::{button, text, Row};
use indexmap::IndexMap;

pub mod tab;
use tab::Tab;

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
    ) -> anyhow::Result<iced::Element<'_, Message, iced::Renderer<iced::Theme>>> {
        // Create top tab view
        let mut tab_list = Row::new();

        for (index, tab) in &self.tabs {
            // create tab as a button
            let tab = button(text(
                // get file name location
                tab.location()
                    .canonicalize()?
                    .file_name()
                    // unwrap ok since name is canonicalized
                    .unwrap()
                    .to_string_lossy(),
            ))
            // focus tab when the button is pressed
            .on_press(Message::Tab(TabMessage::Focus, *index));

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
