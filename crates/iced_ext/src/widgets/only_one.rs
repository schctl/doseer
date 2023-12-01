//! A collection of widgets out of which only one is displayed.

use iced_core::widget::{Operation, Tree};
use iced_core::{mouse, renderer, Element, Length, Widget};

/// A wrapper widget that contains multiple widgets, but displays only one at a time.
pub struct OnlyOne<'a, Message, Renderer> {
    /// Index of the element currently being displayed.
    focused: usize,
    /// All the widgets we contain.
    contents: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> OnlyOne<'a, Message, Renderer> {
    /// Create a new [`OnlyOne`] with the provided contents, focusing on the first one by default.
    #[inline]
    pub fn new(contents: impl Iterator<Item = Element<'a, Message, Renderer>>) -> Self {
        let contents = contents.collect::<Vec<_>>();
        assert!(!contents.is_empty());

        Self {
            focused: 0,
            contents,
        }
    }

    /// Change the currently displayed widget. The `focus` argument is the index of the new element
    /// to display.
    #[inline]
    pub fn focus(mut self, focus: usize) -> Self {
        assert!(focus < self.contents.len());
        self.focused = focus;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for OnlyOne<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn width(&self) -> Length {
        self.contents[self.focused].as_widget().width()
    }

    fn height(&self) -> Length {
        self.contents[self.focused].as_widget().height()
    }

    fn children(&self) -> Vec<Tree> {
        self.contents.iter().map(|e| Tree::new(e)).collect()
    }

    fn diff(&self, state: &mut Tree) {
        state.diff_children(&self.contents);
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        self.contents[self.focused]
            .as_widget()
            .layout(renderer, limits)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_core::Renderer>::Theme,
        style: &renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced_core::Rectangle,
    ) {
        self.contents[self.focused].as_widget().draw(
            &state.children[self.focused],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    // ˅ Some basic container stuff ˅

    fn operate(
        &self,
        state: &mut Tree,
        layout: iced_core::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_core::widget::Operation<Message>,
    ) {
        operation.container(
            None,
            layout.bounds(),
            &mut |operation: &mut dyn Operation<Message>| {
                self.contents[self.focused].as_widget().operate(
                    &mut state.children[self.focused],
                    layout,
                    renderer,
                    operation,
                )
            },
        )
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: iced_core::Event,
        layout: iced_core::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &iced_core::Rectangle,
    ) -> iced_core::event::Status {
        self.contents[self.focused].as_widget_mut().on_event(
            &mut state.children[self.focused],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: iced_core::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced_core::Rectangle,
        renderer: &Renderer,
    ) -> iced_core::mouse::Interaction {
        self.contents[self.focused].as_widget().mouse_interaction(
            &state.children[self.focused],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'call>(
        &'call mut self,
        state: &'call mut Tree,
        layout: iced_core::Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_core::overlay::Element<'call, Message, Renderer>> {
        self.contents[self.focused].as_widget_mut().overlay(
            &mut state.children[self.focused],
            layout,
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<OnlyOne<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
{
    #[inline]
    fn from(value: OnlyOne<'a, Message, Renderer>) -> Self {
        Element::new(value)
    }
}

/// Construct a new [`OnlyOne`].
#[inline]
pub fn only_one<'a, Message, Renderer>(
    contents: impl Iterator<Item = Element<'a, Message, Renderer>>,
) -> OnlyOne<'a, Message, Renderer> {
    OnlyOne::new(contents)
}
