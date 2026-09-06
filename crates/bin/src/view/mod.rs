pub mod main_content;
pub mod space_content;

mod mock {
    use std::collections::HashMap;

    use context::node::{Node, NodeData, NodeId};
    use context::space::{ProfileContext, ProfileId, SidebarContext, SpaceContext};
    use context::tab_data::{ApiRequestData, TabData};
    use http::Method;

    pub fn sidebar_context() -> SidebarContext {
        let data_tab1 = TabData::new_api_tab(
            "localhost/api/ping".to_string(),
            Method::GET,
            HashMap::new(),
            String::new(),
            String::new(),
            HashMap::new(),
        );
        let data_tab2 = TabData::new_api_tab(
            "localhost/api/health".to_string(),
            Method::GET,
            HashMap::new(),
            String::new(),
            String::new(),
            HashMap::new(),
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
                data: TabData::new_api_tab(
                    "localhost/api/users".to_string(),
                    Method::GET,
                    HashMap::from([("page".to_string(), "1".to_string())]),
                    String::new(),
                    String::new(),
                    HashMap::new(),
                ),
                open: true,
            },
            position: 1,
        };
        let folder1_tab2 = Node {
            id: folder1_tab2_id.clone(),
            name: "request2".to_string(),
            data: NodeData::Tab {
                data: TabData::new_api_tab(
                    "localhost/api/teams".to_string(),
                    Method::GET,
                    HashMap::new(),
                    String::new(),
                    String::new(),
                    HashMap::new(),
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
                data: TabData::new_api_tab(
                    "localhost/api/matches".to_string(),
                    Method::GET,
                    HashMap::new(),
                    String::new(),
                    String::new(),
                    HashMap::new(),
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

        // nested folder mock: folder3 -> subfolder1 (tab + subsubfolder) -> subsubfolder (tabs)
        let folder3_node_id = NodeId::default();
        let folder3_sub1_id = NodeId::default();
        let folder3_subsub_id = NodeId::default();
        let folder3_sub1_tab_id = NodeId::default();
        let folder3_subsub_tab1_id = NodeId::default();
        let folder3_subsub_tab2_id = NodeId::default();

        let folder3_sub1_tab = Node {
            id: folder3_sub1_tab_id.clone(),
            name: "get_user".to_string(),
            data: NodeData::Tab {
                data: TabData::new_api_tab(
                    "localhost/api/users/1".to_string(),
                    Method::GET,
                    HashMap::new(),
                    String::new(),
                    String::new(),
                    HashMap::new(),
                ),
                open: false,
            },
            position: 1,
        };
        let folder3_subsub_tab1 = Node {
            id: folder3_subsub_tab1_id.clone(),
            name: "delete_user".to_string(),
            data: NodeData::Tab {
                data: TabData::ApiRequest(ApiRequestData {
                    url: "localhost/api/users/1".to_string(),
                    tab_number: None,
                    favicon: "icons/sidebar-left.svg".to_string(),
                    method: Method::DELETE,
                    params: HashMap::new(),
                    body: String::new(),
                    authorization: "Bearer mock-token".to_string(),
                    headers: HashMap::new(),
                }),
                open: false,
            },
            position: 1,
        };
        let folder3_subsub_tab2 = Node {
            id: folder3_subsub_tab2_id.clone(),
            name: "update_user".to_string(),
            data: NodeData::Tab {
                data: TabData::ApiRequest(ApiRequestData {
                    url: "localhost/api/users/1".to_string(),
                    tab_number: None,
                    favicon: "icons/sidebar-left.svg".to_string(),
                    method: Method::PATCH,
                    params: HashMap::new(),
                    body: r#"{"name": "renamed_user"}"#.to_string(),
                    authorization: "Bearer mock-token".to_string(),
                    headers: HashMap::from([(
                        "Content-Type".to_string(),
                        "application/json".to_string(),
                    )]),
                }),
                open: false,
            },
            position: 2,
        };
        let folder3_subsub = Node {
            id: folder3_subsub_id.clone(),
            name: "subsubfolder".to_string(),
            data: NodeData::Folder {
                children: vec![
                    folder3_subsub_tab1_id.clone(),
                    folder3_subsub_tab2_id.clone(),
                ],
                expand: true,
            },
            position: 2,
        };
        let folder3_sub1 = Node {
            id: folder3_sub1_id.clone(),
            name: "subfolder1".to_string(),
            data: NodeData::Folder {
                children: vec![folder3_sub1_tab_id.clone(), folder3_subsub_id.clone()],
                expand: true,
            },
            position: 1,
        };
        let folder3 = Node {
            id: folder3_node_id.clone(),
            name: "folder3".to_string(),
            data: NodeData::Folder {
                children: vec![folder3_sub1_id.clone()],
                expand: true,
            },
            position: 3,
        };

        let folder_list = vec![
            folder1_node_id.clone(),
            folder2_node_id.clone(),
            folder3_node_id.clone(),
        ];
        nodes.insert(folder1_tab1_id, folder1_tab1);
        nodes.insert(folder1_tab2_id, folder1_tab2);
        nodes.insert(folder1_tab3_id, folder1_tab3);
        nodes.insert(folder2_tab1_id, folder2_tab1);
        nodes.insert(folder1_node_id, folder1);
        nodes.insert(folder2_node_id, folder2);
        nodes.insert(folder3_sub1_tab_id, folder3_sub1_tab);
        nodes.insert(folder3_subsub_tab1_id, folder3_subsub_tab1);
        nodes.insert(folder3_subsub_tab2_id, folder3_subsub_tab2);
        nodes.insert(folder3_subsub_id, folder3_subsub);
        nodes.insert(folder3_sub1_id, folder3_sub1);
        nodes.insert(folder3_node_id, folder3);

        SidebarContext::new()
            .with_nodes(nodes)
            .with_favorite(fav_tab)
            .with_folder(folder_list)
    }

    pub fn api_tab(
        name: &str,
        url: &str,
        method: Method,
        position: u8,
        open: bool,
    ) -> (NodeId, Node) {
        let id = NodeId::default();
        let node = Node {
            id: id.clone(),
            name: name.to_string(),
            data: NodeData::Tab {
                data: TabData::new_api_tab(
                    url.to_string(),
                    method,
                    HashMap::new(),
                    String::new(),
                    String::new(),
                    HashMap::new(),
                ),
                open,
            },
            position,
        };
        (id, node)
    }

    pub fn profile(pseudonym: &str) -> ProfileContext {
        ProfileContext {
            id: ProfileId::new(),
            pseudonym: pseudonym.to_string(),
            email: None,
        }
    }

    pub fn work_sidebar() -> SidebarContext {
        let (fav1_id, fav1) = api_tab(
            "list_repos",
            "api.github.com/user/repos",
            Method::GET,
            1,
            false,
        );
        let (fav2_id, fav2) = api_tab(
            "create_issue",
            "api.github.com/repos/ddeedev/silly/issues",
            Method::POST,
            2,
            false,
        );
        let (t1_id, t1) = api_tab(
            "get_repo",
            "api.github.com/repos/ddeedev/silly",
            Method::GET,
            1,
            false,
        );
        let (t2_id, t2) = api_tab(
            "list_commits",
            "api.github.com/repos/ddeedev/silly/commits",
            Method::GET,
            2,
            false,
        );
        let folder_id = NodeId::default();
        let folder = Node {
            id: folder_id.clone(),
            name: "github".to_string(),
            data: NodeData::Folder {
                children: vec![t1_id.clone(), t2_id.clone()],
                expand: true,
            },
            position: 1,
        };

        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        nodes.insert(fav1_id.clone(), fav1);
        nodes.insert(fav2_id.clone(), fav2);
        nodes.insert(t1_id, t1);
        nodes.insert(t2_id, t2);
        nodes.insert(folder_id.clone(), folder);

        SidebarContext::new()
            .with_nodes(nodes)
            .with_favorite(vec![fav1_id, fav2_id])
            .with_folder(vec![folder_id])
    }

    pub fn lab_sidebar() -> SidebarContext {
        let (fav_id, fav) = api_tab("get_ip", "httpbin.org/ip", Method::GET, 1, false);
        let (t1_id, t1) = api_tab("post_echo", "httpbin.org/post", Method::POST, 1, false);
        let folder_id = NodeId::default();
        let folder = Node {
            id: folder_id.clone(),
            name: "httpbin".to_string(),
            data: NodeData::Folder {
                children: vec![t1_id.clone()],
                expand: false,
            },
            position: 1,
        };

        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        nodes.insert(fav_id.clone(), fav);
        nodes.insert(t1_id, t1);
        nodes.insert(folder_id.clone(), folder);

        SidebarContext::new()
            .with_nodes(nodes)
            .with_favorite(vec![fav_id])
            .with_folder(vec![folder_id])
    }

    pub fn work_space() -> SpaceContext {
        SpaceContext::new("personal".to_string(), 1, profile("profile_1"))
    }

    pub fn spaces() -> Vec<SpaceContext> {
        let mut personal = SpaceContext::new("personal".to_string(), 1, profile("profile_1"));
        personal.sidebar = sidebar_context();

        let mut work = SpaceContext::new("work".to_string(), 2, profile("profile_2"));
        work.sidebar = work_sidebar();

        let mut lab = SpaceContext::new("lab".to_string(), 3, profile("profile_3"));
        lab.sidebar = lab_sidebar();

        vec![personal, work, lab]
    }
}
