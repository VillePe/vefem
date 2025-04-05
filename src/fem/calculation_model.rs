use std::collections::BTreeMap;

use crate::structure::{CalculationElement, Element, Node};

pub struct CalcModel<'a> {
    pub structure_nodes: &'a BTreeMap<i32, Node>,
    pub extra_nodes: BTreeMap<i32, Node>,
    pub structure_elements: &'a Vec<Element>,
    pub calc_elements: Vec<CalculationElement<'a>>,
    all_nodes: BTreeMap<i32, &'a Node>
}

impl<'a> CalcModel<'a> {

    pub fn new(
        structure_nodes: &'a BTreeMap<i32, Node>, 
        extra_nodes: BTreeMap<i32, Node>, 
        structure_elements: &'a Vec<Element>,
        calc_elements: Vec<CalculationElement<'a>>
    ) -> Self {
        Self {
            structure_nodes,
            extra_nodes,
            structure_elements,
            calc_elements,
            all_nodes: BTreeMap::new()
        }
    }

    pub fn get_nodes(&'a self) -> &BTreeMap<i32, &Node> { 
        &self.all_nodes 
    }

    pub fn get_nodes_mut(&'a mut self) -> &BTreeMap<i32, &Node> {
        if self.all_nodes.is_empty() {
            self.all_nodes = BTreeMap::new();
            for n in self.structure_nodes.values() {
                self.all_nodes.insert(n.number, n);
            }
            for n in self.extra_nodes.values() {
                self.all_nodes.insert(n.number, n);
            }            
        }
        &self.all_nodes
    }

    pub fn get_node_count(&self) -> usize {
        self.structure_nodes.len() + self.extra_nodes.len()
    }
}