#![allow(dead_code)]
use crate::structure::node::Node;
use crate::material::*;

pub enum Material {
    Concrete(concrete::Concrete),
    Steel(steel::Steel),
    Timber(timber::Timber),
}

pub struct Element {
    pub node_start: Node,
    pub node_end: Node,
    pub material: Material,
}

impl Element {
    pub fn new(node_start: Node, node_end: Node, material: Material) -> Self {
        Self{node_start, node_end, material}
    }
}