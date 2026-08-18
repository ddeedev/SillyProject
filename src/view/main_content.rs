use gpui::{App, Context, Entity, FocusHandle, Focusable, Window, div, prelude::*, px, rgb};

use crate::action::{Quit, ToggleSidebar};
use crate::components::sidebar::Sidebar;

pub struct MainContent {
    sidebar: Entity<Sidebar>,
    focus_handle: FocusHandle,
}

impl MainContent {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let sidebar = cx.new(Sidebar::new);
        cx.observe(&sidebar, |_this, _sidebar, cx| cx.notify())
            .detach();
        Self {
            sidebar,
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for MainContent {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MainContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar_fraction = self.sidebar.read(cx).visible_fraction();

        div()
            .key_context("main_view")
            .track_focus(&self.focus_handle)
            .on_action(|_: &Quit, window, _| {
                window.remove_window();
            })
            .on_action(cx.listener(|this, _: &ToggleSidebar, window, cx| {
                this.sidebar
                    .update(cx, |sidebar, cx| sidebar.toggle(window, cx));
            }))
            .flex()
            .flex_row()
            .size_full()
            .bg(rgb(0x636080))
            .child(self.sidebar.clone())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .m_2()
                    .ml(px(8.0 * (1.0 - sidebar_fraction)))
                    // 2. The styling for the card stroke containe
                    .bg(rgb(0x7A769F))
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(rgb(0x565375))
                    // Layout and inner children styling
                    .p_4()
                    .gap_3()
                    .justify_center()
                    .items_center()
                    .text_xl()
                    .text_color(rgb(0xffffff))
                    .on_mouse_move(cx.listener(|this, _: &gpui::MouseMoveEvent, _window, cx| {
                        this.sidebar
                            .update(cx, |sidebar, cx| sidebar.set_floating_visible(false, cx));
                    }))
                    .child(div()),
            )
    }
}
