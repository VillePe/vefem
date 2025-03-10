use serde::{Deserialize, Serialize};

use super::element_reinforcement::ReinforcementData;

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleRebar {
    reinf_data: ReinforcementData,
    x: f64,
    y: f64
}