use gpui::{App, IntoElement, Window, deferred, div, prelude::*, px, rgb, svg};
use std::time::Duration;

use crate::platform::set_traffic_lights_hidden;

// Stateful sidebar view. Owns the docked sidebar, the resize handle,
// the hover hot-zone and the floating sidebar overlay.
pub struct Sidebar {
    hidden: bool,
    width: f32,
    width_anim: f32,
    animating: bool,
    floating_visible: bool,
    floating_progress: f32,
    floating_animating: bool,
}

impl Sidebar {
    pub fn new(_cx: &mut gpui::Context<Self>) -> Self {
        Self {
            hidden: false,
            width: 242.0,
            width_anim: 242.0,
            animating: false,
            floating_visible: false,
            floating_progress: 0.0,
            floating_animating: false,
        }
    }

    /// 1.0 when fully open, 0.0 when fully hidden (animated).
    pub fn visible_fraction(&self) -> f32 {
        self.width_anim / self.width
    }

    pub fn toggle(&mut self, cx: &mut gpui::Context<Self>) {
        let hidden = self.hidden;
        self.set_hidden(!hidden, cx);
    }

    pub fn set_hidden(&mut self, hidden: bool, cx: &mut gpui::Context<Self>) {
        if self.hidden == hidden {
            return;
        }
        self.hidden = hidden;
        cx.notify();
        if self.animating {
            return;
        }
        self.animating = true;
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(16))
                    .await;
                match this.update(cx, |this, cx| {
                    let target = if this.hidden { 0.0 } else { this.width };
                    let diff = target - this.width_anim;
                    let settled = diff.abs() < 1.0;
                    this.width_anim = if settled {
                        target
                    } else {
                        this.width_anim + diff * 0.3
                    };
                    cx.notify();
                    settled
                }) {
                    Ok(true) | Err(_) => break,
                    Ok(false) => {}
                }
            }
            let _ = this.update(cx, |this, _| this.animating = false);
        })
        .detach();
    }

    pub fn set_floating_visible(&mut self, visible: bool, cx: &mut gpui::Context<Self>) {
        if self.floating_visible == visible {
            return;
        }
        self.floating_visible = visible;
        cx.notify();
        if self.floating_animating {
            return;
        }
        self.floating_animating = true;
        cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(16))
                    .await;
                match this.update(cx, |this, cx| {
                    let target = if this.floating_visible { 1.0 } else { 0.0 };
                    let diff = target - this.floating_progress;
                    let settled = diff.abs() < 0.02;
                    this.floating_progress = if settled {
                        target
                    } else {
                        this.floating_progress + diff * 0.3
                    };
                    cx.notify();
                    settled
                }) {
                    Ok(true) | Err(_) => break,
                    Ok(false) => {}
                }
            }
            let _ = this.update(cx, |this, _| this.floating_animating = false);
        })
        .detach();
    }
}

#[derive(Clone)]
struct SidebarResizeDrag;

impl Render for SidebarResizeDrag {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div().w(px(4.0)).h_full()
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        set_traffic_lights_hidden(
            window,
            self.width_anim < 1.0 && self.floating_progress <= 0.0,
        );

        let toggle = cx.listener(|this, _: &gpui::ClickEvent, _window, cx| this.toggle(cx));

        let toggle_floating = cx.listener(|this, _: &gpui::ClickEvent, _window, cx| {
            this.set_floating_visible(false, cx);
            this.set_hidden(false, cx);
        });

        let show_floating = cx.listener(|this, _: &gpui::MouseMoveEvent, _window, cx| {
            if this.hidden {
                this.set_floating_visible(true, cx);
            }
        });

        let entity = cx.entity();

        div()
            .h_full()
            .flex_shrink_0()
            .flex()
            .flex_row()
            .on_drag_move(
                move |event: &gpui::DragMoveEvent<SidebarResizeDrag>, _window, cx| {
                    let pos: f32 = event.event.position.x.into();
                    let new_width = pos.clamp(200.0, 500.0);
                    entity.update(cx, |view, cx| {
                        view.width = new_width;
                        if !view.hidden {
                            view.width_anim = new_width;
                        }
                        cx.notify();
                    });
                },
            )
            .when(self.width_anim > 0.0, |el| {
                el.child(
                    div()
                        .w(px(self.width_anim))
                        .h_full()
                        .flex_shrink_0()
                        .overflow_hidden()
                        .flex()
                        .justify_end()
                        .child(AppSidebar::new(false).width(self.width).toggle_sidebar(
                            move |window, cx| {
                                toggle(&gpui::ClickEvent::default(), window, cx);
                            },
                        )),
                )
                .child(
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
            .child(deferred(
                div()
                    .when(self.hidden && self.width_anim <= 0.0, |el| {
                        el.child(
                            div()
                                .id("sidebar-hover-zone")
                                .absolute()
                                .left_0()
                                .top_0()
                                .bottom_0()
                                .w(px(12.0))
                                .on_mouse_move(show_floating),
                        )
                    })
                    .when(
                        self.floating_visible || self.floating_progress > 0.0,
                        |el| {
                            el.child(
                                div()
                                    .id("floating-sidebar")
                                    .absolute()
                                    .left(px(-(1.0 - self.floating_progress) * self.width))
                                    .top_0()
                                    .bottom_0()
                                    .occlude()
                                    .shadow_lg()
                                    .child(
                                        AppSidebar::new(false).width(self.width).toggle_sidebar(
                                            move |window, cx| {
                                                toggle_floating(
                                                    &gpui::ClickEvent::default(),
                                                    window,
                                                    cx,
                                                );
                                            },
                                        ),
                                    ),
                            )
                        },
                    ),
            ))
    }
}
#[derive(IntoElement)]
pub struct AppSidebar {
    hide: bool,
    width: f32,
    on_toggle: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
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
        self.on_toggle = Some(Box::new(f));
        self
    }
}

impl RenderOnce for AppSidebar {
    fn render(mut self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        if self.hide {
            return div();
        }

        let on_toggle = self.on_toggle.take();
        let header = self.render_header_with_toggle(on_toggle);

        div()
            .w(px(self.width))
            .h_full()
            .bg(rgb(0x636080))
            .flex()
            .flex_col()
            .child(header)
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
    fn render_header_with_toggle(
        &self,
        on_toggle: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    ) -> impl IntoElement {
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
                    .path("icons/sidebar-left.svg")
                    .w(px(18.0))
                    .h(px(16.0))
                    .text_color(rgb(0xF8F8F8))
                    .opacity(0.5),
            );

        if let Some(on_toggle) = on_toggle {
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
        // let gap = 8.0_f32;
        // let available = self.width - 20.0;
        // let cols: u32 = if self.width <= 80.0 {
        //     1
        // } else if self.width < 240.0 {
        //     2
        // } else {
        //     3
        // };
        // let item_width = px((available - gap * (cols as f32 - 1.0)) / cols as f32);

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
                    .child(div().opacity(0.5).child((index + 1).to_string()))
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
