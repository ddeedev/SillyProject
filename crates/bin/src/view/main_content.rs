use std::collections::HashMap;

use gpui::{
    App, Context, Entity, FocusHandle, Focusable, SharedString, Window, div, prelude::*, px, rgb,
};
use space::node::{Node, NodeData, NodeId};
use space::tab_data::TabData;

use crate::action::{Quit, ToggleSidebar};
use crate::components::sidebar::SidebarView;
use space::{ProfileId, SidebarContext, SpaceContext};

pub struct MainContent {
    space: Entity<SpaceContext>,
    sidebar: Entity<SidebarView>,
    focus_handle: FocusHandle,
}

impl MainContent {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let space_sidebar = SidebarContext::new();

        let data_tab1 = TabData::new_tab(
            "localhost".to_string(),
            "icons/sidebar-left.svg".to_string(),
        );
        let data_tab2 = TabData::new_tab(
            "localhost".to_string(),
            "icons/sidebar-left.svg".to_string(),
        );

        let tab1_node_id = NodeId::default();
        let tab2_node_id = NodeId::default();
        let fav_tab1 = {
            let id = tab1_node_id.to_string();
            let name = "fav1".to_string();
            let data = NodeData::Tab {
                data: data_tab1,
                open: false,
            };
            Node {
                id: NodeId(id),
                name,
                data,
                position: 1,
            }
        };
        let fav_tab2 = {
            let id = tab2_node_id.to_string();
            let name = "fav2".to_string();
            let data = NodeData::Tab {
                data: data_tab2,
                open: false,
            };
            Node {
                id: NodeId(id),
                name,
                data,
                position: 2,
            }
        };
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        let fav_tab = vec![tab1_node_id.clone(), tab2_node_id.clone()];
        nodes.insert(tab1_node_id, fav_tab1);
        nodes.insert(tab2_node_id, fav_tab2);

        // folder mock: folder1 (expanded, 2 tabs), folder2 (collapsed, 1 tab)
        let folder1_node_id = NodeId::default();
        let folder2_node_id = NodeId::default();
        let folder1_tab1_id = NodeId::default();
        let folder1_tab2_id = NodeId::default();
        let folder2_tab1_id = NodeId::default();

        let folder1_tab1 = Node {
            id: folder1_tab1_id.clone(),
            name: "request1".to_string(),
            data: NodeData::Tab {
                data: TabData::new_tab(
                    "localhost/api/users".to_string(),
                    "icons/sidebar-left.svg".to_string(),
                ),
                open: true,
            },
            position: 1,
        };
        let folder1_tab2 = Node {
            id: folder1_tab2_id.clone(),
            name: "request2".to_string(),
            data: NodeData::Tab {
                data: TabData::new_tab(
                    "localhost/api/teams".to_string(),
                    "icons/sidebar-left.svg".to_string(),
                ),
                open: false,
            },
            position: 2,
        };
        let folder2_tab1 = Node {
            id: folder2_tab1_id.clone(),
            name: "request3".to_string(),
            data: NodeData::Tab {
                data: TabData::new_tab(
                    "localhost/api/matches".to_string(),
                    "icons/sidebar-left.svg".to_string(),
                ),
                open: false,
            },
            position: 1,
        };
        let folder1 = Node {
            id: folder1_node_id.clone(),
            name: "folder1".to_string(),
            data: NodeData::Folder {
                children: vec![folder1_tab1_id.clone(), folder1_tab2_id.clone()],
                expand: true,
            },
            position: 1,
        };
        let folder2 = Node {
            id: folder2_node_id.clone(),
            name: "folder2".to_string(),
            data: NodeData::Folder {
                children: vec![folder2_tab1_id.clone()],
                expand: false,
            },
            position: 2,
        };

        let folder_list = vec![folder1_node_id.clone(), folder2_node_id.clone()];
        nodes.insert(folder1_tab1_id, folder1_tab1);
        nodes.insert(folder1_tab2_id, folder1_tab2);
        nodes.insert(folder2_tab1_id, folder2_tab1);
        nodes.insert(folder1_node_id, folder1);
        nodes.insert(folder2_node_id, folder2);

        let space_sidebar_entity = cx.new(|_| {
            space_sidebar
                .clone()
                .with_nodes(nodes)
                .with_favorite(fav_tab)
                .with_folder(folder_list)
        });

        let space_name: SharedString = "mock".to_string().into();

        let sidebar = cx.new(|_cx| SidebarView::new(_cx, space_name.clone(), space_sidebar_entity));
        cx.observe(&sidebar, |_this, _sidebar, cx| cx.notify())
            .detach();

        let space_ctx = SpaceContext {
            id: space::SpaceId(1.to_string()),
            number: 1,
            profile: space::ProfileContext {
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
