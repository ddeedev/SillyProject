use context::space::{ProfileId, SidebarContext, SpaceContext};
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, SharedString, Window, div, prelude::*, px, rgb,
};
use ui::sidebar::SidebarView;

use crate::action::{Quit, ToggleSidebar};

pub struct MainContent {
    space: Entity<SpaceContext>,
    sidebar: Entity<SidebarView>,
    focus_handle: FocusHandle,
}

impl MainContent {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let space_sidebar = SidebarContext::new();
        let space_sidebar_entity = cx.new(|_| super::mock::sidebar_context());

        let space_name: SharedString = "mock".to_string().into();

        let sidebar = cx.new(|_cx| SidebarView::new(_cx, space_name.clone(), space_sidebar_entity));
        cx.observe(&sidebar, |_this, _sidebar, cx| cx.notify())
            .detach();

        let space_ctx = SpaceContext {
            id: context::space::SpaceId(1.to_string()),
            number: 1,
            profile: context::space::ProfileContext {
                id: ProfileId::new(),
                pseudonym: "profile_1".to_string(),
                email: None,
            },
            name: space_name.to_string(),
            sidebar: space_sidebar,
        };

        let space = cx.new(|_| space_ctx);
        cx.observe(&space, |_this, _space, cx| cx.notify()).detach();

        Self {
            space,
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
            .on_action(cx.listener(|this, _: &ToggleSidebar, _window, cx| {
                this.sidebar.update(cx, |sidebar, cx| sidebar.toggle(cx));
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

pub trait SpaceContent {
    fn init() -> Entity<SpaceContext>;
    fn spawn(name: String, number: u8) -> Entity<SpaceContext>;
}

impl SpaceContent for MainContent {
    fn init() -> Entity<SpaceContext> {
        todo!()
    }

    fn spawn(name: String, number: u8) -> Entity<SpaceContext> {
        todo!()
    }
}
