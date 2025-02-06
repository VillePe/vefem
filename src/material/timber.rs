#![allow(dead_code)]

use crate::material::Steel;

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