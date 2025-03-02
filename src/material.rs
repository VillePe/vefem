mod concrete;
mod steel;
mod timber;

pub use concrete::Concrete;
use serde::{Deserialize, Serialize};
pub use steel::Steel;
pub use timber::Timber;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type", content = "data")]
pub enum MaterialData {
    Concrete(Concrete),
    Steel(Steel),
    Timber(Timber),
}

pub fn get_elastic_modulus(material_type: &MaterialData) -> f64 {
    match material_type {
        MaterialData::Concrete(c) => c.elastic_modulus,
        MaterialData::Steel(s) => s.elastic_modulus,
        MaterialData::Timber(t) => t.elastic_modulus
    }
}

pub fn get_thermal_expansion_coefficient(material_type: &MaterialData) -> f64 {
    match material_type {
        MaterialData::Concrete(c) => c.thermal_expansion_coefficient,
        MaterialData::Steel(s) => s.thermal_expansion_coefficient,
        MaterialData::Timber(t) => t.thermal_expansion_coefficient
    }
}