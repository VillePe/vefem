#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::{ElasticModulus, ThermalExpansion};

#[derive(Debug, Serialize, Deserialize)]
pub struct Steel {
    pub elastic_modulus: f64,
    pub thermal_expansion_coefficient: f64,
    pub yield_strength: f64,
    pub break_strength: f64,
}

impl Steel {
    pub fn new(elastic_modulus: f64) -> Self {
        Self {
            elastic_modulus,
            ..Self::default()
        }
    }

    pub fn new_s355() -> Self {
        Self { ..Self::default() }
    }

    pub fn new_s235() -> Self {
        Self {
            yield_strength: 235.0,
            break_strength: 360.0,
            ..Self::default()
        }
    }

    pub fn new_aisi304() -> Self {
        Self {
            elastic_modulus: 193.0,
            yield_strength: 205.0,
            break_strength: 515.0,
            thermal_expansion_coefficient: 17.3,
            ..Self::default()
        }
    }

    pub fn new_aisi314() -> Self {
        Self {
            elastic_modulus: 196.0,
            yield_strength: 230.0,
            break_strength: 550.0,
            thermal_expansion_coefficient: 17.3,
            ..Self::default()
        }
    }
}

impl Default for Steel {
    /// Default values from S355
    fn default() -> Self {
        // Default thermal coefficient got from
        // https://www.engineeringtoolbox.com/linear-expansion-coefficients-d_95.html
        Self {
            elastic_modulus: 210e3,
            thermal_expansion_coefficient: 12.5e-6,
            yield_strength: 355.0,
            break_strength: 510.0,
        }
    }
}

impl ThermalExpansion for Steel {
    fn get_thermal_expansion_coefficient(&self) -> f64 {
        self.thermal_expansion_coefficient
    }
}

impl ElasticModulus for Steel {
    fn get_elastic_modulus(&self) -> f64 {
        self.elastic_modulus
    }
}