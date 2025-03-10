use serde::{Deserialize, Serialize};

use super::element_reinforcement::ReinforcementTrait;

#[derive(Debug, Serialize, Deserialize)]
pub struct RebarData {
    pub char_strength: f64,
    pub elastic_modulus: f64
}

impl ReinforcementTrait for RebarData {
    fn get_char_strength(&self) -> f64 {
        self.char_strength
    }

    fn get_elastic_modulus(&self) -> f64 {
        self.elastic_modulus
    }
}