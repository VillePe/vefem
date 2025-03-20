use vputilslib::{equation_handler::EquationHandler, geometry2d};

use crate::{material::Concrete, settings::CalculationSettings};

use super::PolygonProfile;

pub fn area_from_radius(radius: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2)
}

pub fn area_from_diameter(diameter: f64) -> f64 {
    std::f64::consts::PI * diameter.powi(2) / 4.0
}

/// Gets the area of the profile in square millimeters (mm²)
pub fn get_area_conc_section(profile: &PolygonProfile, concrete: &Concrete, _calc_settings: &CalculationSettings) -> f64 {
    // Only the polygon type is calculated. Other types have constant values.
    let mut area = geometry2d::calculate_area(&profile.polygon);
    for r in &concrete.reinforcement.main_rebars {
        for s in r.get_calculation_rebars(profile, &EquationHandler::new()) {
            let reduced_area = s.area * (s.reinf_data.get_elastic_modulus() / concrete.elastic_modulus - 1.0); // Note that the 'hole' is taken into account
            area += reduced_area;                        
        }
    }
    area
}