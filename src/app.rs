use gpui::{
    App, Application, Bounds, Context, FocusHandle, Focusable, KeyBinding, SharedString,
    TitlebarOptions, Window, WindowBackgroundAppearance, WindowBounds, WindowKind, WindowOptions,
    div, point, prelude::*, px, rgb, size,
};
use gpui_component::{ActiveTheme, theme};
use std::rc::Rc;

use crate::platform::set_traffic_lights_hidden;

use crate::action::{Quit, ToggleSidebar};
use crate::component::sidebar::AppSidebar;

struct HelloWorld {
    text: SharedString,
    sidebar_hidden: bool,
    focus_handle: FocusHandle,
}

impl Focusable for HelloWorld {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
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

        div()
            .key_context("main_view")
            .track_focus(&self.focus_handle)
            .on_action(|_: &Quit, window, _| {
                window.remove_window();
            })
            .on_action(move |event: &ToggleSidebar, _window, cx| {
                keybind_action(event, _window, cx);
            })
            .flex()
            .flex_row()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(AppSidebar::new(self.sidebar_hidden).toggle_sidebar({
                let toggle_action = toggle_action.clone();
                move |window, cx| {
                    toggle_action(&gpui::ClickEvent::default(), window, cx);
                }
            }))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .h_full()
                    .bg(rgb(0x505050))
                    // titlebar strip for main content — draggable, shows toggle when sidebar is hidden
                    .child(
                        div()
                            .h(px(52.0))
                            .w_full()
                            .flex()
                            .items_end()
                            .pb_2()
                            .when(self.sidebar_hidden, |el| {
                                el.pl(px(76.0)).child(
                                    gpui_component::button::Button::new("Show Sidebar").on_click({
                                        let toggle_action = toggle_action.clone();
                                        move |_, window, cx| {
                                            toggle_action(&gpui::ClickEvent::default(), window, cx);
                                        }
                                    }),
                                )
                            })
                            .on_mouse_move(|_, window, _| {
                                window.start_window_move();
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .gap_3()
                            .justify_center()
                            .items_center()
                            .shadow_lg()
                            .border_1()
                            .border_color(rgb(0x0000ff))
                            .text_xl()
                            .text_color(rgb(0xffffff))
                            .child(format!("Hello, {}!", &self.text))
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .size_8()
                                            .bg(gpui::red())
                                            .border_1()
                                            .border_dashed()
                                            .rounded_md()
                                            .border_color(gpui::white()),
                                    )
                                    .child(
                                        div()
                                            .size_8()
                                            .bg(gpui::green())
                                            .border_1()
                                            .border_dashed()
                                            .rounded_md()
                                            .border_color(gpui::white()),
                                    )
                                    .child(
                                        div()
                                            .size_8()
                                            .bg(gpui::blue())
                                            .border_1()
                                            .border_dashed()
                                            .rounded_md()
                                            .border_color(gpui::white()),
                                    )
                                    .child(
                                        div()
                                            .size_8()
                                            .bg(gpui::yellow())
                                            .border_1()
                                            .border_dashed()
                                            .rounded_md()
                                            .border_color(gpui::white()),
                                    )
                                    .child(
                                        div()
                                            .size_8()
                                            .bg(gpui::black())
                                            .border_1()
                                            .border_dashed()
                                            .rounded_md()
                                            .border_color(gpui::white()),
                                    )
                                    .child(
                                        div()
                                            .size_8()
                                            .bg(gpui::white())
                                            .border_1()
                                            .border_dashed()
                                            .rounded_md()
                                            .border_color(gpui::black()),
                                    ),
                            ),
                    ),
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
                        focus_handle,
                    }
                })
            })
            .unwrap();
        });
    }
}
