//! Try to cheaply load desktop specific configurations.

mod gnome {
    use std::process::Command;

    /// Try to load the current GTK theme via the `gsettings` binary. Jank but works I guess.
    pub fn current_theme() -> Option<String> {
        Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "icon-theme"])
            .output()
            .ok()
            .and_then(|o| {
                std::str::from_utf8(&o.stdout)
                    .ok()
                    .map(|s| s.replace(['\'', '"', '\n'], ""))
            })
    }
}

mod kde {
    use std::process::Command;

    /// Try to load the current QT5 theme via `kreadconfig`. Just as jank as GTK version.
    pub fn current_theme() -> Option<String> {
        Command::new("kreadconfig5")
            .args(["--file", "kdeglobals", "--group", "Icons", "--key", "Theme"])
            .output()
            .ok()
            .and_then(|o| {
                std::str::from_utf8(&o.stdout)
                    .ok()
                    .map(|s| s.replace(['\'', '"'], ""))
            })
    }
}

/// Try to load the current theme from the desktop environment.
pub fn current_theme() -> Option<String> {
    [gnome::current_theme, kde::current_theme]
        .into_iter()
        .find_map(|f| (f)())
}
