use context::space::SpaceContext;
use gpui::Entity;
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
