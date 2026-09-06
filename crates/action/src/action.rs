use crate::keybind;
use gpui::{Action, App, actions};

actions!(app, [ToggleSidebar, Quit, CloseTab, ResetSidebar]);

#[derive(Clone, Debug, PartialEq, Action)]
#[action(namespace = app, no_json)]
pub struct SwitchSpace(pub usize);
