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
            .flex()
            .flex_col()
            .child(
                div()
                    .h(px(52.0))
                    .w_full()
                    .flex()
                    .items_end()
                    .pb_2()
                    .pl(px(76.0))
                    .on_mouse_move(|_, window, _| {
                        window.start_window_move();
                    })
                    .child(toggle_btn),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p_4()
                    .gap_4()
                    .child(div().child("Sidebar Content")),
            )
    }
}
