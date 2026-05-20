use gpui::{App, IntoElement, Window, div, prelude::*, px, rgb, svg};
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
        if self.hide {
            return div();
        }

        div()
            .w(px(self.width))
            .h_full()
            .bg(rgb(0x636080))
            .flex()
            .flex_col()
            .child(self.render_header())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .m_2()
                    .mr_1()
                    .gap_2()
                    .child(self.render_workspace_card())
                    .child(self.favorite_tap())
                    .child(div().gap_2().flex_1().child(self.folder_request()))
                    .child(self.space_selection()),
            )
    }
}

impl AppSidebar {
    fn render_header(&self) -> impl IntoElement {
        let mut toggle_btn = div()
            .id("sidebar-toggle")
            .w(px(30.0))
            .h(px(28.0))
            .rounded(px(6.0))
            .cursor_pointer()
            .flex()
            .justify_center()
            .items_center()
            .hover(|style| style.bg(rgb(0x7A769F)))
            .child(
                svg()
                    .path("icon/sidebar-left.svg")
                    .w(px(18.0))
                    .h(px(16.0))
                    .text_color(rgb(0xF8F8F8))
                    .opacity(0.5),
            );

        if let Some(on_toggle) = &self.on_toggle {
            let on_toggle = on_toggle.clone();
            toggle_btn = toggle_btn.on_click(move |_, window, ctx| {
                on_toggle(window, ctx);
            });
        }

        div()
            .mt_2()
            .w_full()
            .flex()
            .items_center()
            .justify_end()
            .pb_2()
            .on_mouse_move(|_, window, _| {
                window.start_window_move();
            })
            .child(toggle_btn)
    }

    fn render_workspace_card(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap_2()
            .bg(rgb(0x7A769F))
            .rounded(px(6.0))
            .border_color(rgb(0x565375))
            .text_xl()
            .text_center()
            .text_color(rgb(0xF8F8F8))
            .opacity(0.5)
            .font_family("Pacifico")
            .child(div().child("⭐"))
            .child(
                div()
                    .child("WorkSpace")
                    .font_weight(gpui::FontWeight::EXTRA_BOLD),
            )
    }

    fn favorite_tap(&self) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_row()
            .flex_wrap()
            .justify_start()
            .gap_2()
            .text_xl()
            .text_color(rgb(0xF8F8F8))
            .children((0..9).map(move |index| {
                let element_id: gpui::SharedString = format!("grid-item-{}", index).into();
                div()
                    .id(element_id)
                    .w_full()
                    .when(self.width <= 212., |el| el.w_full())
                    .when(self.width > 212. && self.width <= 242., |el| {
                        el.w(gpui::relative(0.48))
                    })
                    .when(self.width / 3.0 > 80.0, |el| el.w(gpui::relative(0.31)))
                    .h(px(60.0))
                    .bg(rgb(0x7A769F))
                    .rounded(px(10.0))
                    .cursor_pointer()
                    .flex()
                    .justify_center()
                    .items_center()
                    .hover(|style| style.bg(rgb(0x565375)))
                    .child(div().opacity(0.5).child(format!("{}", index + 1)))
            }))
    }

    fn folder_request(&self) -> impl IntoElement {
        div().w_full().h_full().bg(rgb(0xffffff))
    }

    fn space_selection(&self) -> impl IntoElement {
        div()
            .flex_shrink_0()
            .h(px(30.0))
            .bg(rgb(0x7A769F))
            .rounded(px(6.0))
    }
}
