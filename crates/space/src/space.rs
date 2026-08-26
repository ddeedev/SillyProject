use crate::node::{Node, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod node;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpaceId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    pub id: SpaceId,
    pub name: String,
    pub favorites: Vec<NodeId>,
    pub pinned: Vec<NodeId>,
    pub folder: Vec<NodeId>,
    pub root_folder: Vec<NodeId>,
    pub open_folder: HashMap<NodeId, bool>,
    pub nodes: HashMap<NodeId, Node>,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
