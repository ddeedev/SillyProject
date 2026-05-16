use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use gpui_component::theme;

use crate::component::sidebar::AppSidebar;

struct HelloWorld {
    text: SharedString,
    sidebar_hidden: bool,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let toggle_action = cx.listener(|this, _event: &gpui::ClickEvent, _window, cx| {
            this.sidebar_hidden = !this.sidebar_hidden;
            cx.notify();
        });

        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(
                AppSidebar::new(self.sidebar_hidden).toggle_sidebar(move |window, cx| {
                    toggle_action(&gpui::ClickEvent::default(), window, cx);
                }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .gap_3()
                    .bg(rgb(0x505050))
                    .h_full()
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
            )
    }
}

pub struct SetupApp;

impl SetupApp {
    pub fn run() {
        Application::new().run(|cx: &mut App| {
            theme::init(cx);
            let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| HelloWorld {
                        text: "World".into(),
                        sidebar_hidden: false,
                    })
                },
            )
            .unwrap();
        });
    }
}
