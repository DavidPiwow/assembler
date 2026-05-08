use core::fmt;
use std::collections::HashMap;

use crate::parser::node::LCNode;


pub type NodeVec = Vec<Box<dyn LCNode>>;
pub type LabelMap = HashMap<String, u16>;

pub struct Program {
    tree: NodeVec,
    labels: LabelMap,
}

impl Program {
    pub fn from(tree: NodeVec, labels: LabelMap) -> Self {
        Self {
            tree, labels
        }
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for instr in &self.tree {
            let _ = write!(f, "{instr:?}\n");
        }
        write!(f, "{:?}", self.labels)
    }
}