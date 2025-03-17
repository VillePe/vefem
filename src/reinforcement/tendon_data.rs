use serde::{Deserialize, Serialize};

use super::ReinforcementTrait;


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct TendonData {
    pub yield_strength: f64,
    pub elastic_modulus: f64,
    pub prestress: f64,
    // NOTE! These are just inital properties. They might change at a later point. Needs testing
    // to determine if these are a good idea.
    pub tension_at_release: f64,
    pub tension_at_install: f64,
    pub tension_at_load_applied: f64,
    pub tension_at_long_term: f64,
}

impl ReinforcementTrait for TendonData {
    fn get_yield_strength(&self) -> f64 {
        self.yield_strength
    }

    fn get_elastic_modulus(&self) -> f64 {
        self.elastic_modulus
    }
}