use std::vec;

use super::mock;
use crate::view::{
    mock::{work_sidebar, work_space},
    space_content::SpaceContent,
};
use action::action::{Quit, ResetSidebar, SwitchSpace, ToggleSidebar};
use context::space::{ProfileId, SidebarContext, SpaceContext};
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Global, SharedString, Window, div, prelude::*,
    px, rgb,
};
use ui::sidebar::SidebarView;

impl Global for MainContent {}

pub struct MainContent {
    pub current_space: usize,
    pub spaces: Vec<SpaceContent>,
    pub space_logos: Vec<String>,
    pub focus_handle: FocusHandle,
}

impl MainContent {
    pub fn global(cx: &App) -> &Self {
        cx.global()
    }

    // TODO::
    // Using db or local space file instead of hardcode
    pub fn new(cx: &mut Context<Self>) -> Self {
        let mut logos: Vec<String> = Vec::new();

        let space_sidebar = SidebarContext::new();
        let space_sidebar_entity = cx.new(|_| mock::sidebar_context());
        let space_name: SharedString = "mock".to_string().into();
        if let Some(logo) = space_name.chars().nth(0) {
            logos.push(logo.to_string());
        }

        let sidebar = cx.new(|_cx| SidebarView::new(space_name.clone(), space_sidebar_entity));
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

        let default_space = SpaceContent { space, sidebar };

        let work_space = work_space();
        if let Some(logo) = work_space.name.chars().nth(0) {
            logos.push(logo.to_string());
        }

        let work_sidebar = work_sidebar();
        let sidebar_ctx = cx.new(|_| work_sidebar);
        cx.observe(&sidebar_ctx, |_this, _sidebar, cx| cx.notify())
            .detach();

        let sidebar_view = SidebarView::new(work_space.name.clone().into(), sidebar_ctx);
        let work_space_entity = cx.new(|_| work_space);
        cx.observe(&work_space_entity, |_this, _sidebar, cx| cx.notify())
            .detach();

        let space_content = SpaceContent::new(work_space_entity, cx.new(|_| sidebar_view));

        Self {
            current_space: 1,
            spaces: vec![default_space.clone(), space_content.clone()],
            focus_handle: cx.focus_handle(),
            space_logos: logos,
        }
    }

    pub fn new_space(&mut self, cx: &mut Context<Self>, space_ctn: SpaceContent) {
        if let Some(logo) = space_ctn.space.read(cx).name.chars().nth(0) {
            self.space_logos.push(logo.to_string());
        }
        self.spaces.push(space_ctn);
        self.current_space = self.spaces.len();
        cx.notify();
    }

    pub fn with_space(&mut self, cx: &mut Context<Self>, space_ctns: Vec<SpaceContent>) {
        for space_ctn in space_ctns.into_iter() {
            if let Some(logo) = space_ctn.space.read(cx).name.chars().nth(0) {
                self.space_logos.push(logo.to_string());
            }
            self.spaces.push(space_ctn);
        }
        self.current_space = 1;
        cx.notify();
    }

    pub fn switch_space(&mut self, cx: &mut Context<Self>, number: usize) {
        if number > 0 && number <= self.spaces.len() {
            self.current_space = number;
            cx.notify();
        }
    }

    pub fn current_sidebar(&self) -> Entity<SidebarView> {
        self.spaces[self.current_space - 1].sidebar.clone()
    }

    pub fn retrive_logos(&self) -> Vec<String> {
        self.space_logos.clone()
    }
}

impl Focusable for MainContent {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MainContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current_sidebar = self.current_sidebar();
        let space_logos = self.retrive_logos();
        current_sidebar.update(cx, |sidebar, sb_cx| {
            sidebar.register_logos(sb_cx, space_logos)
        });

        let sidebar_fraction = current_sidebar.read(cx).visible_fraction();

        div()
            .key_context("main_view")
            .track_focus(&self.focus_handle)
            .on_action(|_: &Quit, window, _| {
                window.remove_window();
            })
            .on_action(cx.listener(|this, _: &ToggleSidebar, _window, cx| {
                this.current_sidebar()
                    .update(cx, |sidebar, sb_cx| sidebar.toggle(sb_cx));
            }))
            .on_action(cx.listener(|this, action: &SwitchSpace, _window, cx| {
                this.switch_space(cx, action.0);
            }))
            .on_action(cx.listener(|this, _: &ResetSidebar, _window, cx| {
                this.current_sidebar()
                    .update(cx, |sidebar, sb_cx| sidebar.reset(sb_cx));
            }))
            .flex()
            .flex_row()
            .size_full()
            .bg(rgb(0x636080))
            .child(current_sidebar)
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
                        this.current_sidebar()
                            .update(cx, |sidebar, cx| sidebar.set_floating_visible(false, cx));
                    })),
            )
            .child(div().flex_col().flex_col())
    }
}
