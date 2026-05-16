use crate::action::ToggleSidebar;
use gpui::{AnyElement, App, Entity, IntoElement, KeyBinding, Style, Window, div, prelude::*, px};
use gpui_component::button::Button;
use gpui_component::input::InputState;
use gpui_component::{ActiveTheme, Icon};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct AppSidebar {
    // state
    hide: bool,
    // event
    on_toggle: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl AppSidebar {
    pub fn new(hide: bool) -> Self {
        Self {
            hide,
            on_toggle: None,
        }
    }

    pub fn toggle_sidebar(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for AppSidebar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut toggle_btn = Button::new("Hide Sidebar");

        if let Some(on_toggle) = self.on_toggle {
            toggle_btn = toggle_btn.on_click(move |_, window, ctx| {
                on_toggle(window, ctx);
            })
        }

        if self.hide {
            return div().child(toggle_btn);
        }

        div()
            .w(px(240.0))
            .h_full()
            .bg(gpui::rgb(0x202020))
            .p_4()
            .flex()
            .flex_col()
            .gap_4()
            .child(toggle_btn)
            .child(div().child("Sidebar Content"))
    }
}
