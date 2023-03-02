//! Side bar widget.

use std::path::Path;

use doseer_core::config::Config;
use doseer_core::path::PathWrap;

use iced::widget::{button, column, container, row, text, Column};
use iced::{alignment, Alignment, Color, Length, Padding};
use iced_lazy::Component;
use sleet::ColorScheme;

use crate::gui::{self, Element};
use crate::{pane, theme, Icon};

/// Sidebar events.
#[derive(Debug, Clone)]
pub enum Message {
    /// An event that indicates a message needs to be sent to the current pane.
    Pane(pane::Message),
}

/// Creates the sidebar component.
#[inline]
pub const fn side_bar<IsOpen>(config: &Config, is_open: IsOpen) -> SideBar<IsOpen>
where
    IsOpen: Fn(&Path) -> bool,
{
    SideBar::new(config, is_open)
}

/// The file picker side bar.
#[derive(Debug)]
pub struct SideBar<'conf, IsOpen>
where
    IsOpen: Fn(&Path) -> bool,
{
    /// User configuration.
    config: &'conf Config,
    /// Location open check.
    is_open: IsOpen,
}

impl<'conf, IsOpen> SideBar<'conf, IsOpen>
where
    IsOpen: Fn(&Path) -> bool,
{
    #[inline]
    pub const fn new(config: &'conf Config, is_open: IsOpen) -> Self {
        Self { config, is_open }
    }
}

impl<'conf, IsOpen> Component<gui::Message, gui::Renderer> for SideBar<'conf, IsOpen>
where
    IsOpen: Fn(&Path) -> bool,
{
    type State = ();
    type Event = Message;

    fn update(&mut self, _: &mut Self::State, event: Self::Event) -> Option<gui::Message> {
        match event {
            Message::Pane(p) => Some(gui::Message::Pane(p)),
        }
    }

    fn view(&self, _: &Self::State) -> Element<'_, Self::Event> {
        let title = container(text("Files").font(theme::fonts::UI::Black).size(28))
            .height(pane::Pane::TOP_BAR_HEIGHT)
            .align_y(alignment::Vertical::Center)
            .padding([0, 8]);

        // Bookmarks column
        let mut col = Column::new()
            .align_items(Alignment::Center)
            .padding(8)
            .spacing(4);

        for path in &self.config.bookmarks {
            col = col.push(item_button(path, &self.is_open));
        }

        // TODO: Network column, Other locations

        container(column!(title, col))
            .style(theme::container::Container::Weak)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn item_button(path: &PathWrap, is_open: impl Fn(&Path) -> bool) -> Element<Message> {
    button(
        row!(
            Icon::Directory
                .svg()
                .width(Length::Fixed(22.0))
                .height(Length::Fixed(22.0)),
            text(path.display().to_string_lossy())
                .size(22)
                .font(theme::fonts::UI::Regular),
        )
        .spacing(6)
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Fill),
    )
    // focus tab when the button is pressed
    .on_press(Message::Pane(pane::Message::Replace(path.clone())))
    .width(Length::Fill)
    .height(Length::Fixed(38.0))
    .padding(Padding::from([4, 8]))
    .style(
        if (is_open)(path) {
            ButtonStyle::Focused
        } else {
            ButtonStyle::Default
        }
        .into(),
    )
    .into()
}

/// Tab button theme.
#[derive(Debug, Clone, Default)]
pub enum ButtonStyle {
    #[default]
    Default,
    Focused,
}

impl From<ButtonStyle> for theme::button::Button {
    fn from(t: ButtonStyle) -> Self {
        Self::SideBar(t)
    }
}

impl ButtonStyle {
    pub fn active(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
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
                text_color: palette.primary.base.on_base,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }

    pub fn hovered(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
        let palette = theme.palette();

        match self {
            Self::Focused => self.active(theme),
            Self::Default => iced::widget::button::Appearance {
                background: palette.surface.weak.base.into(),
                text_color: palette.surface.weak.on_base,
                border_radius: 6.0,
                ..Default::default()
            },
        }
    }

    pub fn pressed(&self, theme: &theme::Theme) -> iced::widget::button::Appearance {
        Self::Focused.active(theme)
    }
}
