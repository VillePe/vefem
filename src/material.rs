mod concrete;
mod steel;
mod timber;

pub use concrete::Concrete;
pub use steel::Steel;
pub use timber::Timber;
use crate::structure::element::MaterialType;

pub fn get_elastic_modulus(material_type: &MaterialType) -> f64 {
    match material_type {
        MaterialType::Concrete(c) => {c.elastic_modulus}
        MaterialType::Steel(s) => {s.elastic_modulus}
        MaterialType::Timber(t) => {t.elastic_modulus}
    }
}

pub fn get_thermal_expansion_coefficient(material_type: &MaterialType) -> f64 {
    match material_type {
        MaterialType::Concrete(c) => {c.thermal_expansion_coefficient }
        MaterialType::Steel(s) => {s.thermal_expansion_coefficient}
        MaterialType::Timber(t) => {t.thermal_expansion_coefficient}
    }
}