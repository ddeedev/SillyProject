use gpui::{
    App, Application, Bounds, Context, FocusHandle, Focusable, KeyBinding, SharedString,
    TitlebarOptions, TouchPhase, Window, WindowBackgroundAppearance, WindowBounds, WindowKind,
    WindowOptions, div, point, prelude::*, px, rgb, size,
};
use gpui_component::{ActiveTheme, theme};
use std::rc::Rc;

use crate::platform::set_traffic_lights_hidden;

use crate::action::{Quit, ToggleSidebar};
use crate::component::sidebar::AppSidebar;

struct HelloWorld {
    text: SharedString,
    sidebar_hidden: bool,
    sidebar_width: f32,
    focus_handle: FocusHandle,
}

impl Focusable for HelloWorld {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Clone)]
struct SidebarResizeDrag;

impl Render for SidebarResizeDrag {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div().w(px(4.0)).h_full()
    }
}

impl Render for HelloWorld {
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
                        .w(px(5.0))
                        .h_full()
                        .cursor_col_resize()
                        .hover(|s| s.bg(rgb(0x555555)))
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
                    .ml_2()
                    .bg(rgb(0x7A769F))
                    .rounded(px(12.0))
                    .border_1()
                    .border_color(rgb(0x565375))
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

pub struct AppRunner;

impl AppRunner {
    pub fn run() {
        Application::new().run(|cx: &mut App| {
            theme::init(cx);
            cx.on_action(|_: &Quit, cx| cx.quit());
            cx.bind_keys([
                KeyBinding::new("cmd-q", Quit, None),
                KeyBinding::new("cmd-s", ToggleSidebar, Some("main_view")),
            ]);

            cx.on_window_closed(|cx| {
                if cx.windows().is_empty() {
                    cx.quit();
                }
            })
            .detach();
            let titlebar = TitlebarOptions {
                appears_transparent: true,
                title: None,
                traffic_light_position: Some(point(px(12.0), px(20.0))),
            };
            let window_option = WindowOptions {
                titlebar: Some(titlebar),
                kind: WindowKind::Normal,
                is_movable: true,
                window_background: WindowBackgroundAppearance::Opaque,
                ..Default::default()
            };

            cx.open_window(window_option, |window, cx| {
                cx.new(|cx| {
                    let focus_handle = cx.focus_handle();
                    window.focus(&focus_handle);
                    HelloWorld {
                        text: "World".into(),
                        sidebar_hidden: false,
                        sidebar_width: 240.0,
                        focus_handle,
                    }
                })
            })
            .unwrap();
        });
    }
}
