//! The GUI app.

use doseer_colorschemes::default::Wrap;
use doseer_iced_ext::components::panelled::{self, unpanelled};

use iced::widget::component;
use iced::{executor, Application, Command, Length};

use crate::content::{self, Content};
use crate::side_bar::side_bar;
use crate::{config, item, tab, theme, Config, Theme};

/// Shorthand for an iced element generic over some message.
pub type Renderer = iced::Renderer<Wrap<Theme>>;
pub type Element<'a, T> = iced::Element<'a, T, Renderer>;

#[derive(Debug, Clone)]
pub enum Message {
    Content(content::Message),
    Config(config::Message),
    ResizeMain(panelled::pane_grid::ResizeEvent),
    IcedEvent(iced::Event),
    FontLoad(Result<(), iced::font::Error>),
}

/// The UI state.
pub struct Gui {
    /// Sidebar split tracker.
    split_state: panelled::State,
    /// Main content.
    content: Content,
    /// Configuration.
    config: Config,
}

impl Application for Gui {
    type Executor = executor::Default;
    type Flags = Config;
    type Message = Message;
    type Theme = Wrap<Theme>;

    fn new(config: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut commands = vec![];

        // initialize the content area
        let content = Content::new();

        let mut split_state = panelled::State::new();
        split_state.resize(0.2);

        // tab update watcher
        let tab_init_cmd = tab::watcher::command(content.focused().location());
        commands.push(
            tab_init_cmd
                .map(content::Message::tab)
                .map(Message::Content),
        );

        // load all the fonts we need
        commands.push(theme::fonts::load_all().map(Message::FontLoad));

        (
            Self {
                split_state,
                config,
                content,
            },
            Command::batch(commands),
        )
    }

    fn title(&self) -> String {
        format!("{}", item::item_name(self.content.focused().location()))
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::events().map(Message::IcedEvent)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let mut commands = vec![];

        match message {
            Message::Content(m) => {
                let pane_cmd = self.content.update(m).unwrap();
                commands.push(pane_cmd.map(Message::Content));
            }
            Message::ResizeMain(m) => self.split_state.resize(m.ratio),
            Message::Config(m) => self.config.process_message(m),
            _ => {}
        }

        Command::batch(commands)
    }

    fn view(&self) -> Element<Self::Message> {
        unpanelled(|| self.content.view().map(Message::Content))
            // add side panel
            .panel(&self.split_state, |_| {
                component(side_bar(&self.config, |path| {
                    self.content.focused().location().as_ref() == path
                }))
            })
            // configure inner pane_grid
            .into_inner()
            .width(Length::Fill)
            .height(Length::Fill)
            .on_resize(16, Message::ResizeMain)
            .into()
    }
}

impl Drop for Gui {
    fn drop(&mut self) {
        match self.config.flush() {
            Ok(_) => tracing::info!("flushed configuration file"),
            Err(e) => tracing::error!("failed to write configuration: {:?}", e),
        }
    }
}
