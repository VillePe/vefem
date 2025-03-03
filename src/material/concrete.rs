#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use super::MaterialTrait;

#[derive(Debug, Serialize, Deserialize)]
pub struct Concrete {
    pub elastic_modulus: f64,
    pub thermal_expansion_coefficient: f64,
    /// The characteristic strength of the concrete (f_ck)
    pub char_strength: f64
}

impl Concrete {
    pub fn new() -> Self {
        Self{ ..Self::default() }
    }

    pub fn standard(t: StandardConcrete) -> Self {
        t.value()
    }
}

pub enum StandardConcrete {
    C12_15,
    C16_20,
    C20_25,
    C25_30,
    C30_37,
    C35_45,
    C40_50,
    C45_55,
    C50_60,
    C55_67,
    C60_75,
    C70_85,
    C80_95,
    C90_105
}
impl StandardConcrete {
    fn value(&self) -> Concrete {
        match *self {
            StandardConcrete::C12_15 => new_with_char_strength(12.0),
            StandardConcrete::C16_20 => new_with_char_strength(16.0),
            StandardConcrete::C20_25 => new_with_char_strength(20.0),
            StandardConcrete::C25_30 => new_with_char_strength(25.0),
            StandardConcrete::C30_37 => new_with_char_strength(30.0),
            StandardConcrete::C35_45 => new_with_char_strength(35.0),
            StandardConcrete::C40_50 => new_with_char_strength(40.0),
            StandardConcrete::C45_55 => new_with_char_strength(45.0),
            StandardConcrete::C50_60 => new_with_char_strength(50.0),
            StandardConcrete::C55_67 => new_with_char_strength(55.0),
            StandardConcrete::C60_75 => new_with_char_strength(60.0),
            StandardConcrete::C70_85 => new_with_char_strength(70.0),
            StandardConcrete::C80_95 => new_with_char_strength(80.0),
            StandardConcrete::C90_105 => new_with_char_strength(90.0),
        }
    }
}

pub fn new_with_char_strength(char_strength: f64) -> Concrete {
    Concrete{ char_strength, elastic_modulus: calc_elastic_modulus(char_strength), ..Default::default() }
}

/// Calculates the elastic modulus of concrete by EN 1992-1-1 (E_cm, MPa)
/// * `char_strength` - The characteristic strength of the concrete (f_ck, MPa)
pub fn calc_elastic_modulus(char_strength: f64) -> f64 {
    let fcm = calc_mean_compressive_strength(char_strength);    
    22.0 * (fcm / 10.0).powf(0.3)
}

/// Calculates the mean compressive strength of concrete by EN 1992-1-1 (fcm, MPa)
/// * `char_strength` - The characteristic strength of the concrete (f_ck, MPa)
pub fn calc_mean_compressive_strength(char_strength: f64) -> f64 {
    char_strength + 8.0
}


impl Default for Concrete {
    fn default() -> Self {
        // Default thermal coefficient got from 
        // https://www.engineeringtoolbox.com/linear-expansion-coefficients-d_95.html
        Self { char_strength: 12.0, elastic_modulus: 27e3, thermal_expansion_coefficient: 14.0e-6 }
    }
}

impl MaterialTrait for Concrete {
    fn get_thermal_expansion_coefficient(&self) -> f64 {
        self.thermal_expansion_coefficient
    }
    fn get_elastic_modulus(&self) -> f64 {
        self.elastic_modulus
    }
}