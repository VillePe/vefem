use serde::{Deserialize, Serialize};

use super::ReinforcementTrait;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct RebarData {
    pub yield_strength: f64,
    pub elastic_modulus: f64
}

impl RebarData {
    pub fn new(yield_strength: f64, elastic_modulus: f64) -> Self {
        Self {
            yield_strength,
            elastic_modulus
        }
    }


    pub fn new_b500b() -> Self {
        Self {
            yield_strength: 500.0,
            elastic_modulus: 200000.0,
        }
    }
}

impl ReinforcementTrait for RebarData {
    fn get_yield_strength(&self) -> f64 {
        self.yield_strength
    }

    fn get_elastic_modulus(&self) -> f64 {
        self.elastic_modulus
    }
}