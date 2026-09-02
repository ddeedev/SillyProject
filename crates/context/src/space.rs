use crate::node::{Node, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpaceId(pub String);

impl SpaceId {
    pub fn new() -> Self {
        SpaceId(Uuid::new_v4().to_string())
    }
}

impl Default for SpaceId {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceContext {
    pub id: SpaceId,
    // number use for cmd + number to switch space
    pub name: String,
    pub number: u8,
    pub profile: ProfileContext,
    pub sidebar: SidebarContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileContext {
    pub id: ProfileId,
    // user_name// name tag
    pub pseudonym: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidebarContext {
    pub folder: Vec<NodeId>,
    pub favorites: Vec<NodeId>,
    pub pinned: Vec<NodeId>,
    pub nodes: HashMap<NodeId, Node>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProfileId(pub String);

impl ProfileId {
    pub fn new() -> Self {
        ProfileId(Uuid::new_v4().to_string())
    }
}

impl Default for ProfileId {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileContext {
    pub fn new(name: String, email: Option<String>) -> Self {
        Self {
            id: ProfileId::default(),
            pseudonym: name,
            email,
        }
    }
}

impl SidebarContext {
    pub fn new() -> Self {
        Self {
            folder: Vec::new(),
            favorites: Vec::new(),
            pinned: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn with_folder(self, folder: Vec<NodeId>) -> Self {
        Self { folder, ..self }
    }

    pub fn with_pinned_tab(self, pinned: Vec<NodeId>) -> Self {
        Self { pinned, ..self }
    }

    pub fn with_favorite(self, favorites: Vec<NodeId>) -> Self {
        Self { favorites, ..self }
    }

    pub fn with_nodes(self, nodes: HashMap<NodeId, Node>) -> Self {
        Self { nodes, ..self }
    }
}

impl Default for SidebarContext {
    fn default() -> Self {
        Self::new()
    }
}

impl SpaceContext {
    pub fn new(name: String, number: u8, profile: ProfileContext) -> Self {
        Self {
            id: SpaceId::default(),
            name,
            number,
            profile,
            sidebar: SidebarContext::default(),
        }
    }

}
