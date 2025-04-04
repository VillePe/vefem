use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::{loads::{Load, LoadCombination}, settings::CalculationSettings};
use super::{Element, Node};

#[derive(Debug, Serialize, Deserialize)]
pub struct StructureModel {
    /// All the nodes in the model. The key is the node number
    pub nodes: BTreeMap<i32, Node>,
    /// All the elements in the model
    pub elements: Vec<Element>,
    /// All the load combinations for the calculation model. If it is empty, all the loads
    /// will be used in the calculations without any factors for their strengths
    pub load_combinations: Vec<LoadCombination>,
    /// All the loads that are used in the calculation
    pub loads: Vec<Load>,
    /// The calculation settings
    pub calc_settings: CalculationSettings,
}