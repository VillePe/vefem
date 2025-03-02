pub mod element;
mod node;
mod release;
mod support;
pub mod utils;

use std::collections::HashMap;

pub use element::Element;
pub use node::Node;
pub use release::Release;
pub use support::Support;

use serde::{Deserialize, Serialize};
use crate::{loads::{Load, LoadCombination}, settings::CalculationSettings};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculationModel {
    pub nodes: HashMap<i32, Node>,
    pub elements: Vec<Element>,
    pub load_combinations: Vec<LoadCombination>,
    pub loads: Vec<Load>,
    pub calc_settings: CalculationSettings,
}