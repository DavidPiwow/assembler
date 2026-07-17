// Do not touch, David's

use core::fmt;
use std::collections::HashMap;

use crate::parser::node::LCNode;


pub type NodeVec = Vec<Box<dyn LCNode>>;
pub type LabelMap = HashMap<String, u16>;

pub struct Program {
    tree: NodeVec,
    labels: LabelMap,
    start_location: u16,
    binary: Vec<u16>,
}

impl Program {
    pub fn from(tree: NodeVec, labels: LabelMap, start_location: u16) -> Self {
        Self {
            tree, labels, start_location, binary: vec![],
        }
    }

    pub fn to_binary(&mut self) {
        self.binary.clear();
        let mut instr_pos = 0;

        for node in &mut self.tree {
            let n = node.as_mut();
            let name = n.get_label_name();

            if name.is_some() {
                let label_location = self.labels.get(name.unwrap());
                if label_location.is_some() {
                    n.init_label(*label_location.unwrap() as i16 - (instr_pos+1));
                }
            }

            self.binary.push(node.to_binary());
            instr_pos += 1;
        }
    }
    
    pub fn get_binary(&self) -> &Vec<u16> {
        &self.binary
    }

    pub fn get_start(&self) -> u16 {
        self.start_location
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

