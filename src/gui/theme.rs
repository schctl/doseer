//! Global theme.

sleet::theme!(Theme);

sleet::impl_stylesheets! {
    [Application, Button, Container, PaneGrid, Svg, Scrollable, Text] for Theme;
}
