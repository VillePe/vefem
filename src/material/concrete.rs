#![allow(dead_code)]

use crate::material::Steel;

pub struct Concrete {
    pub elastic_modulus: f64,
    pub thermal_expansion_coefficient: f64,
}

impl Concrete {
    pub  fn new(elastic_modulus: f64) -> Self {
        Self { elastic_modulus, ..Self::default() }
    }
}

impl Default for Concrete {
    fn default() -> Self {
        // Default thermal coefficient got from 
        // https://www.engineeringtoolbox.com/linear-expansion-coefficients-d_95.html
        Self { elastic_modulus: 27e3, thermal_expansion_coefficient: 14.0e-6 }
    }
}