use crate::parser::node::LCNode;

pub struct Program {
    tree: Vec<Box<dyn LCNode>>,
}