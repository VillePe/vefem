use serde::{Deserialize, Serialize};

use super::element_reinforcement::ReinforcementData;

#[derive(Debug, Serialize, Deserialize)]
pub struct RebarRow {
    pub reinf_data: ReinforcementData,
    pub distribution: RebarDistribution,
    pub offset_start: f64,
    pub offset_end: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RebarDistribution {
    Real{diam: f64, count: isize},
    Distributed{diam: f64, distr: f64},
    ByArea{area: f64}
}

impl RebarRow {
    pub fn get_single_rebars(&self) {

    }
}