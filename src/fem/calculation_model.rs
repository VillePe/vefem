use std::collections::BTreeMap;

use crate::structure::{CalculationElement, Node};

pub struct CalcModel<'a> {
    pub nodes: &'a BTreeMap<i32, Node>,
    pub extra_nodes: BTreeMap<i32, Node>,
    pub calc_elements: Vec<CalculationElement>,
}