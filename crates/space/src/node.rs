use crate::tab_data::TabData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeData {
    Folder { children: Vec<NodeId>, expand: bool },
    Tab { data: TabData, open: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    pub parent: Option<NodeId>,
    pub data: NodeData,
}

impl NodeId {
    pub fn new() -> Self {
        NodeId(Uuid::new_v4().to_string())
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Folder {
    fn has_child(&self) -> bool;
    fn is_expand(&self) -> bool;
    fn is_last_node(&self) -> bool;
    fn is_active(&self) -> bool;
}
