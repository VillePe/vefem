use std::collections::BTreeMap;

use crate::structure::{CalculationElement, Element, Node};

pub struct CalcModel<'a> {
    /// The structure nodes
    pub structure_nodes: &'a BTreeMap<i32, Node>,
    /// Extra nodes created when splitting structure elements into calculation elements
    pub extra_nodes: BTreeMap<i32, Node>,
    /// The structure elements
    pub structure_elements: &'a Vec<Element>,
    /// The calculation elements map. The key is the model element number for the calculation elements
    pub calc_elements: BTreeMap<i32, Vec<CalculationElement<'a>>>,
}

impl<'a> CalcModel<'a> {

    pub fn new(
        structure_nodes: &'a BTreeMap<i32, Node>, 
        extra_nodes: BTreeMap<i32, Node>, 
        structure_elements: &'a Vec<Element>,
        calc_elements: BTreeMap<i32, Vec<CalculationElement<'a>>>
    ) -> Self {
        Self {
            structure_nodes,
            extra_nodes,
            structure_elements,
            calc_elements,
        }
    }

    pub fn get_all_calc_elements(&'a self) -> Vec<&CalculationElement> {
        let mut result: Vec<&CalculationElement> = Vec::new();
        for (_, v) in &self.calc_elements {
            for e in v.iter() {
                result.push(e);
            }
        }
        result
    }

    pub fn get_all_nodes(&'a self) -> Vec<&Node> {
        let mut result: Vec<&Node> = Vec::new();
        for (_, n) in self.structure_nodes {
            result.push(n);
        }
        for (_, n) in &self.extra_nodes {
            result.push(n);
        }
        result
    }

    pub fn get_node_count(&self) -> usize {
        self.structure_nodes.len() + self.extra_nodes.len()
    }
}