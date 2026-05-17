use crate::action::ToggleSidebar;
use gpui::{
    AnyElement, App, Entity, IntoElement, KeyBinding, Style, Window, div, prelude::*, px, rgb,
};
use gpui_component::button::Button;
use gpui_component::input::InputState;
use gpui_component::{ActiveTheme, Icon};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct AppSidebar {
    // state
    hide: bool,
    width: f32,
    // event
    on_toggle: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl AppSidebar {
    pub fn new(hide: bool) -> Self {
        Self {
            hide,
            width: 240.0,
            on_toggle: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
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
            return div();
        }

        div()
            .w(px(self.width))
            .h_full()
            .bg(gpui::rgb(0x636080))
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
                div().flex().flex_col().flex_1().p_2().gap_2().child(
                    div()
                        .bg(rgb(0x7A769F))
                        .rounded(px(6.0))
                        .border_color(rgb(0x565375))
                        .text_xl()
                        .text_center()
                        .justify_center()
                        .items_center()
                        .text_color(rgb(0xF8F8F8))
                        .opacity(0.5)
                        .child("Sidebar Content"),
                ),
            )
    }
}
