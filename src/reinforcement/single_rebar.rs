use serde::{Deserialize, Serialize};

use super::reinforcement::ReinforcementData;

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleRebar {
    reinf_data: ReinforcementData,
    x: f64,
    y: f64,
    offset_start: f64,
    offset_end: f64,
}