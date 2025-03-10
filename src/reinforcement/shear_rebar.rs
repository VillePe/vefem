use serde::{Deserialize, Serialize};

use super::element_reinforcement::ReinforcementData;


#[derive(Debug, Serialize, Deserialize)]
pub struct ShearRebarGroup {
    pub reinf_data: ReinforcementData,
    pub distribution: f64,
    pub offset_start: f64,
    pub offset_end: f64,
}