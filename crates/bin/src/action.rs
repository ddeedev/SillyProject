use crate::{keybind, view::main_content::MainContent};
use gpui::{Action, App, actions};

actions!(app, [ToggleSidebar, Quit, CloseTab, ResetSidebar]);

#[derive(Clone, Debug, PartialEq, Action)]
#[action(namespace = app, no_json)]
pub struct SwitchSpace(pub usize);

pub fn register(cx: &mut App) {
    keybind::register_keybind(cx);

    //let spaces = MainContent::global(cx).spaces.clone();
}
