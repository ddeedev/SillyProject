use crate::node::{Node, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub mod node;
pub mod tab_data;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpaceId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    pub id: SpaceId,
    pub profile: ProfileId,
    pub name: String,
    pub favorites: Vec<NodeId>,
    pub pinned: Vec<NodeId>,
    pub folder: Vec<NodeId>,
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
