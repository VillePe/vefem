#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::{ElasticModulus, ThermalExpansion};

#[derive(Debug, Serialize, Deserialize)]
pub struct Timber {
    pub elastic_modulus: f64,
    pub thermal_expansion_coefficient: f64,
}

impl Timber {
    pub fn new() -> Self {
        Self{ ..Self::default() }
    }
    
    pub fn new_c18() -> Self {
        Self{ ..Self::default() }
    }

    pub fn new_c24() -> Self {
        Self{ elastic_modulus: 11e3, ..Self::default() }
    }
}

impl Default for Timber {
    /// Default values from S355
    fn default() -> Self {
        // Default thermal coefficient got from (wood, pine). Elastic modulus is from 
        // "Eurokoodi 5 lyhennetty suunnitteluohje" "Sahatavara C18"
        // https://www.engineeringtoolbox.com/linear-expansion-coefficients-d_95.html
        Self { elastic_modulus: 9e3, thermal_expansion_coefficient: 5.0e-6 }
    }
}

impl ThermalExpansion for Timber {
    fn get_thermal_expansion_coefficient(&self) -> f64 {
        self.thermal_expansion_coefficient
    }
}

impl ElasticModulus for Timber {
    fn get_elastic_modulus(&self) -> f64 {
        self.elastic_modulus
    }
}