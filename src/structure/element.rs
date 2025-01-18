#![allow(dead_code)]

use std::any::Any;
use crate::material::Material;
use crate::structure::node::Node;

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