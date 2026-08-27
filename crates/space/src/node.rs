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
    fn is_tab(&self) -> bool;
    fn list_child(&self) -> Vec<NodeId>;
}

impl Folder for Node {
    fn is_tab(&self) -> bool {
        match &self.data {
            NodeData::Tab { data: _, open: _ } => true,
            NodeData::Folder {
                children: _,
                expand: _,
            } => false,
        }
    }

    fn has_child(&self) -> bool {
        match &self.data {
            NodeData::Tab { data: _, open: _ } => false,
            NodeData::Folder {
                children: _,
                expand: _,
            } => true,
        }
    }

    fn list_child(&self) -> Vec<NodeId> {
        let mut child_node = Vec::new();
        match !&self.is_tab() {
            true => match &self.data {
                NodeData::Folder {
                    children: child,
                    expand: _,
                } => {
                    child.iter().for_each(|c| {
                        child_node.push(c.clone().to_owned());
                    });
                    child_node
                }
                _ => child_node,
            },
            false => child_node,
        }
    }
}
