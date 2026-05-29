use gpui::{App, Context, FocusHandle, Focusable, Window, div, prelude::*, px, rgb};

use crate::platform::set_traffic_lights_hidden;

use crate::action::{Quit, ToggleSidebar};
use crate::component::sidebar::AppSidebar;

pub struct MainContent {
    sidebar_hidden: bool,
    sidebar_width: f32,
    focus_handle: FocusHandle,
}

impl MainContent {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            sidebar_hidden: false,
            sidebar_width: 242.0,
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

        let toggle = cx.listener(|this, _: &gpui::ClickEvent, _window, cx| {
            this.sidebar_hidden = !this.sidebar_hidden;
            cx.notify();
        });

        let keybind_toggle = cx.listener(|this, _: &ToggleSidebar, _window, cx| {
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
            .on_action(move |event: &ToggleSidebar, window, cx| {
                keybind_toggle(event, window, cx);
            })
            .on_drag_move(
                move |event: &gpui::DragMoveEvent<SidebarResizeDrag>, _window, cx| {
                    let pos: f32 = event.event.position.x.into();
                    let new_width = pos.clamp(200.0, 500.0);
                    entity.update(cx, |view, cx| {
                        view.sidebar_width = new_width;
                        println!("sidebar width: {:.1}px", view.sidebar_width);
                        cx.notify();
                    });
                },
            )
            .flex()
            .flex_row()
            .size_full()
            .bg(rgb(0x636080))
            .child(
                AppSidebar::new(self.sidebar_hidden)
                    .width(self.sidebar_width)
                    .toggle_sidebar(move |window, cx| {
                        toggle(&gpui::ClickEvent::default(), window, cx);
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
                    .ml(if self.sidebar_hidden {
                        px(8.0)
                    } else {
                        px(0.0)
                    })
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
                    .child(div()),
            )
    }
}
