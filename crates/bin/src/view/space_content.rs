use super::mock;
use context::space::{ProfileId, SidebarContext, SpaceContext, SpaceId};
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, SharedString, Window, div, prelude::*, px, rgb,
};
use ui::sidebar::SidebarView;

#[derive(Debug, Clone)]
pub struct SpaceContent {
    pub space: Entity<SpaceContext>,
    pub sidebar: Entity<SidebarView>,
}

impl SpaceContent {

    pub fn new(space: Entity<SpaceContext>, sidebar: Entity<SidebarView>) -> Self {
        Self { space, sidebar }
    }
}
