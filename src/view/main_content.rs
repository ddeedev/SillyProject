use gpui::{App, Context, FocusHandle, Focusable, SharedString, Window, div, prelude::*, px, rgb};
use std::rc::Rc;

use crate::platform::set_traffic_lights_hidden;

use crate::action::{Quit, ToggleSidebar};
use crate::component::sidebar::AppSidebar;

pub struct MainContent {
    text: SharedString,
    sidebar_hidden: bool,
    sidebar_width: f32,
    focus_handle: FocusHandle,
}

impl MainContent {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            sidebar_hidden: false,
            sidebar_width: 242.0,
            text: "Hello".into(),
            focus_handle: cx.focus_handle(),
        }
    }
}

#[derive(Clone)]
struct SidebarResizeDrag;

impl Render for SidebarResizeDrag {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div().w(px(4.0)).h_full()
    }
}

impl Focusable for MainContent {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MainContent {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        set_traffic_lights_hidden(window, self.sidebar_hidden);
        let toggle_action = Rc::new(cx.listener(|this, _event: &gpui::ClickEvent, _window, cx| {
            this.sidebar_hidden = !this.sidebar_hidden;
            cx.notify();
        }));

        let keybind_action = cx.listener(|this, _event: &ToggleSidebar, _window, cx| {
            this.sidebar_hidden = !this.sidebar_hidden;
            cx.notify();
        });

        let entity = cx.entity();

        div()
            .key_context("main_view")
            .track_focus(&self.focus_handle)
            .on_action(|_: &Quit, window, _| {
                window.remove_window();
            })
            .on_action(move |event: &ToggleSidebar, _window, cx| {
                keybind_action(event, _window, cx);
            })
            .on_drag_move(
                move |event: &gpui::DragMoveEvent<SidebarResizeDrag>, _window, cx| {
                    let pos: f32 = event.event.position.x.into();
                    let new_width = pos.clamp(200.0, 500.0);
                    entity.update(cx, |view, cx| {
                        view.sidebar_width = new_width;
                        cx.notify();
                    });
                },
            )
            .flex()
            .flex_row()
            .size_full()
            // line divider sidebar <> main
            .bg(rgb(0x636080))
            .child(
                AppSidebar::new(self.sidebar_hidden)
                    .width(self.sidebar_width)
                    .toggle_sidebar({
                        let toggle_action = toggle_action.clone();
                        move |window, cx| {
                            toggle_action(&gpui::ClickEvent::default(), window, cx);
                        }
                    }),
            )
            .when(!self.sidebar_hidden, |el| {
                el.child(
                    div()
                        .id("sidebar-resize-handle")
                        .w(px(3.0))
                        .h_full()
                        .cursor_col_resize()
                        .hover(|s| s.bg(rgb(0x827e7e)))
                        .on_drag(SidebarResizeDrag, |_, _, _, cx| {
                            cx.new(|_| SidebarResizeDrag)
                        }),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .m_2()
                    .when(!self.sidebar_hidden, |el| el.ml_0())
                    .when(self.sidebar_hidden, |el| el.ml_2())
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
                    .child(format!("Hello, {}!", &self.text))
                    .child(div()),
            )
    }
}
