#![allow(dead_code)]


use crate::{
    loads::load::{self, CalculationLoad}, settings::CalculationSettings, structure::CalculationElement
};

use crate::results::NodeResults;

/// Calculates the deflection at given point.
pub fn calculate_at(
    x: f64,
    element: &CalculationElement,
    loads: &Vec<CalculationLoad>,
    settings: &CalculationSettings,
    results: &NodeResults,
) -> f64 {
    // See theory files for more information how these values are calculated.
    // Main idea is to gather the second integral of moment function at x from loads and support reactions about Y-axis.

    // The sum of double integrals from loads and support reactions
    let mut d_integral = 0.0;
    let local_reactions = results.get_elem_local_reactions(element);
    let local_displacements = results.get_elem_local_displacements(element);

    let e_m = element.elastic_modulus;
    let s_mom_area = element.profile.get_major_second_mom_of_area(&element.material, settings);

    for load in loads {
        if load.element_number != element.calc_el_num {
            continue;
        }
        // The factor to handle skewed loads
        let z_dir_factor = load.rotation.to_radians().sin();
        match load.load_type {
            load::CalculationLoadType::Point => {
                if load.offset_start <= x {
                    // Notice that z_dir_factor is negative when pointing downwards => no need to
                    // subtract it from the d_integral, because it already has the correct sign
                    d_integral +=
                        load.strength * z_dir_factor * (x - load.offset_start).powi(3) / 6.0;
                }
            }
            load::CalculationLoadType::Rotational => {
                if load.offset_start <= x {
                    d_integral -= load.strength * (x - load.offset_start).powi(2) / 2.0;
                }
            }
            load::CalculationLoadType::Line => {
                if load.offset_start <= x {
                    let load_length = x - load.offset_start;
                    d_integral += load.strength * z_dir_factor * load_length.powi(4) / 24.0;
                    if load.offset_end <= x {
                        // The imaginary load to cancel the extra load after loads end and before x
                        d_integral -=
                            load.strength * z_dir_factor * (x - load.offset_end).powi(4) / 24.0;
                    }
                }
            }
            load::CalculationLoadType::Triangular => {
                // Triangular load with max load at left hand side
                if load.offset_start < load.offset_end {
                    d_integral += handle_triang_ltr(load, x)
                } else {
                    d_integral += handle_triang_rtl(load, x)
                }
            }
            load::CalculationLoadType::Strain => {}
        };
    }

    // C1 (the rotation about Y-axis times EI and x)
    d_integral += local_displacements[(2, 0)] * e_m * s_mom_area * x;
    // C2 (the displacement in Z-axis times EI)
    d_integral += local_displacements[(1, 0)] * e_m * s_mom_area;
    // Double integral from the support reactions at the start of the element
    d_integral += local_reactions[(1, 0)] * x.powi(3) / 6.0;
    d_integral -= local_reactions[(2, 0)] * x.powi(2) / 2.0;

    // The deflection value
    d_integral / (e_m * s_mom_area)
}

/// Calculates the moment at x for a triangular load with the maximum load at the left hand side.
/// ltr = Left to right
fn handle_triang_ltr(load: &CalculationLoad, x: f64) -> f64 {
    let z_dir_factor = load.rotation.to_radians().sin();
    if load.offset_start <= x {
        if load.offset_end <= x {
            // Load strength shrinks and ends before x
            // The length of the load
            let ll = load.offset_end - load.offset_start;
            // The original triangular load from start to x
            let mut d_integral =
                load.strength / ll * z_dir_factor * (x - load.offset_start).powi(5) * 2.0 / 120.0;
            // The imaginary load after triangular loads end point (this is also triangular)
            d_integral +=
                load.strength / ll * z_dir_factor * (x - load.offset_end).powi(5) * 1.0 / 120.0;
            // The imaginary line load with the strength of triangular load which would go from loads start to x
            d_integral -=
                load.strength / ll * z_dir_factor * (x - load.offset_start).powi(5) / 40.0;
            // The imaginary line load with the strength of triangular load
            d_integral += load.strength * z_dir_factor * (x - load.offset_start).powi(4) / 24.0;
            return d_integral;
        } else {
            // Load strength shrinks and ends after x (needs to be split into triangular and line loads)
            // Split the load into a line load and a triangular load at x.
            let ll = load.offset_end - load.offset_start;
            // Double intgeral of the moment from triangular load
            let mut d_integral =
                load.strength / ll * z_dir_factor * (x - load.offset_start).powi(5) * 2.0 / 120.0;
            // The imaginary line load with the strength of smaller triangular load
            d_integral -=
                load.strength / ll * z_dir_factor * (x - load.offset_start).powi(5) / 40.0;
            // The imaginary line load with the strength of triangular load
            d_integral += load.strength * z_dir_factor * (x - load.offset_start).powi(4) / 24.0;
            return d_integral;
        }
    }
    0.0
}

/// Calculates the moment at x for a triangular load with the maximum load at the right hand side.
/// ltr = Left to right
fn handle_triang_rtl(load: &CalculationLoad, x: f64) -> f64 {
    let z_dir_factor = load.rotation.to_radians().sin();
    // Load offsets at left or right hand side
    let left = load.offset_end;
    let right = load.offset_start;
    if left <= x {
        if right <= x {
            let ll = right - left;
            let mut d_integral = load.strength / ll * z_dir_factor * (x - left).powi(5) / 120.0;
            d_integral -= load.strength * z_dir_factor * (x - right).powi(4) / 24.0;
            d_integral -= load.strength / ll * z_dir_factor * (x - right).powi(5) / 120.0;
            return d_integral;
        } else {
            // Split the load at x. No need to split into a line load, because of
            let ll = right - left;
            let d_integral = load.strength / ll * z_dir_factor * (x - left).powi(5) * 1.0 / 120.0;
            return d_integral;
        }
    }
    0.0
}
