pub mod main_content;

mod mock {
    use std::collections::HashMap;

    use context::node::{Node, NodeData, NodeId};
    use context::space::SidebarContext;
    use context::tab_data::{ApiRequestData, TabData};
    use http::Method;

    pub fn sidebar_context() -> SidebarContext {
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
        let tab3_node_id = NodeId::default();
        let fav_tab3 = {
            let id = tab3_node_id.to_string();
            let name = "fav3".to_string();
            let data = NodeData::Tab {
                data: TabData::ApiRequest(ApiRequestData {
                    url: "localhost/api/users".to_string(),
                    tab_number: None,
                    favicon: "icons/sidebar-left.svg".to_string(),
                    method: Method::GET,
                    params: HashMap::from([("page".to_string(), "1".to_string())]),
                    body: String::new(),
                    authorization: String::new(),
                    headers: HashMap::from([(
                        "Accept".to_string(),
                        "application/json".to_string(),
                    )]),
                }),
                open: true,
            };
            Node {
                id: NodeId(id),
                name,
                data,
                position: 3,
            }
        };
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        let fav_tab = vec![
            tab1_node_id.clone(),
            tab2_node_id.clone(),
            tab3_node_id.clone(),
        ];
        nodes.insert(tab1_node_id, fav_tab1);
        nodes.insert(tab2_node_id, fav_tab2);
        nodes.insert(tab3_node_id, fav_tab3);

        // folder mock: folder1 (expanded, 3 tabs), folder2 (collapsed, 1 tab)
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
        let folder1_tab3_id = NodeId::default();
        let folder1_tab3 = Node {
            id: folder1_tab3_id.clone(),
            name: "create_user".to_string(),
            data: NodeData::Tab {
                data: TabData::ApiRequest(ApiRequestData {
                    url: "localhost/api/users".to_string(),
                    tab_number: None,
                    favicon: "icons/sidebar-left.svg".to_string(),
                    method: Method::POST,
                    params: HashMap::new(),
                    body: r#"{"name": "mock_user"}"#.to_string(),
                    authorization: "Bearer mock-token".to_string(),
                    headers: HashMap::from([(
                        "Content-Type".to_string(),
                        "application/json".to_string(),
                    )]),
                }),
                open: false,
            },
            position: 3,
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
                children: vec![
                    folder1_tab1_id.clone(),
                    folder1_tab2_id.clone(),
                    folder1_tab3_id.clone(),
                ],
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
        nodes.insert(folder1_tab3_id, folder1_tab3);
        nodes.insert(folder2_tab1_id, folder2_tab1);
        nodes.insert(folder1_node_id, folder1);
        nodes.insert(folder2_node_id, folder2);

        SidebarContext::new()
            .with_nodes(nodes)
            .with_favorite(fav_tab)
            .with_folder(folder_list)
    }
}
