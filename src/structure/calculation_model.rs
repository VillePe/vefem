use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{loads::{Load, LoadCombination}, settings::CalculationSettings};
use super::{Element, Node};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculationModel {
    pub nodes: HashMap<i32, Node>,
    pub elements: Vec<Element>,
    pub load_combinations: Vec<LoadCombination>,
    pub loads: Vec<Load>,
    pub calc_settings: CalculationSettings,
}