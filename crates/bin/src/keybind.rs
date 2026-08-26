use crate::action::{Quit, ToggleSidebar};
use gpui::{App, KeyBinding};

pub fn register_keybind(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-s", ToggleSidebar, Some("main_view")),
    ]);
}
