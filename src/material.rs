mod concrete;
mod steel;
mod timber;

pub use concrete::Concrete;
pub use concrete::ConcreteCalcType;
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

impl MaterialData {
    pub fn value(&self) -> &dyn MaterialTrait {
        match self {
            MaterialData::Concrete(c) => c,
            MaterialData::Timber(t) => t,
            MaterialData::Steel(s) => s,
        }
    }
}

impl Default for MaterialData {
    fn default() -> Self {
        MaterialData::Steel(Steel::default())
    }
}

pub fn get_elastic_modulus(material_type: &dyn MaterialTrait) -> f64 {
    material_type.get_elastic_modulus()
}

pub fn get_thermal_expansion_coefficient(material_type: &dyn MaterialTrait) -> f64 {
    material_type.get_thermal_expansion_coefficient()
}

pub trait MaterialTrait {
    fn get_thermal_expansion_coefficient(&self) -> f64;
    fn get_elastic_modulus(&self) -> f64;
}