// Do not touch, David's

use core::fmt;
use std::collections::HashMap;

use crate::parser::{node::{LCNode, Operation}, scanner::TokenError};

// im not typing all that x
pub type NodeVec = Vec<Box<dyn LCNode>>;
pub type LabelMap = HashMap<String, u16>;


/// Represents an LC3 program
pub struct Program {
    /// A collection of nodes, sort of like an Abstract Syntax Tree but not really
    tree: NodeVec,
    /// The easiest way to represent labels is just with a hash map
    labels: LabelMap,
    /// The program needs to be loaded into a specific memory location specified by the
    /// program's author. Usually x3000
    start_location: u16,
    /// A collection of binary instructions and program data that the CPU can load in
    binary: Vec<u16>,
}

impl Program {
    pub fn from(tree: NodeVec, labels: LabelMap, start_location: u16) -> Self {
        Self {
            tree,
            labels,
            start_location,
            binary: vec![],
        }
    }

    pub fn to_binary(&mut self) -> Result<(), TokenError> {
        self.binary.clear();
        let mut instr_pos = 0;

        for node in &mut self.tree {
            // println!("{:?}", node);   // commenting out to silence the debug output for now - Sneha 
            let n = node.as_mut();
            let name = n.get_label_name();
            if name.is_some() {
                // println!("{}", name.unwrap());   // commenting out to silence the debug output - Sneha
                match n.get_operation() {
                    Operation::Literal => {
                        let label_location = self.labels.get(name.unwrap());

                        if label_location.is_some() {
                            n.init_label(*label_location.unwrap() as i16);
                        }
                    }
                    _ => {
                        let label_location = self.labels.get(name.unwrap());
                        if label_location.is_some() {
                            n.init_label(*label_location.unwrap() as i16 - (instr_pos + 1));
                        } else {
                            return Err(TokenError::UnknownLabel(name.unwrap().to_string()));
                        }
                    }
                }
            }
            self.binary.push(node.to_binary());
            instr_pos += 1;
        }

        Ok(())
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
