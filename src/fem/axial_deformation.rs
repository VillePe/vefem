#![allow(dead_code)]

use std::collections::BTreeMap;

use crate::{
    loads::load::{self, CalculationLoad}, settings::CalculationSettings, structure::{Element, Node}
};

use crate::results::NodeResults;

/// Calculates the deflection at given point.
pub fn calculate_at(
    x: f64,
    element: &Element,
    nodes: &BTreeMap<i32, Node>,
    loads: &Vec<CalculationLoad>,
    settings: &CalculationSettings,
    results: &NodeResults,
) -> f64 {
    // See theory files for more information how these values are calculated.
    // Main idea is to gather the first integral of axial force function at x from loads and support reactions in X-axis.

    // The sum of integrals from loads and support reactions
    let mut s_integral = 0.0;
    let local_reactions = results.get_elem_local_reactions(element, nodes);
    let local_displacements = results.get_elem_local_displacements(element, nodes);

    let e_m = element.get_elastic_modulus();
    let area = element.profile.get_area(&element.material, settings);

    for load in loads {
        // The factor to handle skewed loads
        let x_dir_factor = load.rotation.to_radians().cos();
        match load.load_type {
            load::CalculationLoadType::Point => {
                if load.offset_start <= x {
                    s_integral -= load.strength * x_dir_factor * (x - load.offset_start);
                }
            }
            load::CalculationLoadType::Rotational => {}
            load::CalculationLoadType::Line => {
                if load.offset_start <= x {
                    let load_length = x - load.offset_start;
                    s_integral -= load.strength * x_dir_factor * load_length * load_length / 2.0;
                    if load.offset_end <= x {
                        // The imaginary load to cancel the extra load after loads end and before x
                        s_integral +=
                            load.strength * x_dir_factor * (x - load.offset_end).powi(2) / 2.0;
                    }
                }
            }
            load::CalculationLoadType::Triangular => {
                // Triangular load with max load at left hand side
                if load.offset_start < load.offset_end {
                    s_integral -= handle_triang_ltr(load, x)
                } else {
                    s_integral -= handle_triang_rtl(load, x)
                }
            }
            load::CalculationLoadType::Strain => {}
        };
    }

    // C1 (the node translation in X-axis times EA)
    s_integral += local_displacements[(0, 0)] * e_m * area;
    // Double integral from the support reactions at the start of the element
    s_integral -= local_reactions[(0, 0)] * x;

    // The deflection value
    s_integral / (e_m * area)
}

/// Calculates the integral of the axial force function at x for a triangular load with the maximum load at the left hand side.
/// ltr = Left to right
fn handle_triang_ltr(load: &CalculationLoad, x: f64) -> f64 {
    let x_dir_factor = load.rotation.to_radians().cos();
    if load.offset_start <= x {
        if load.offset_end <= x {
            // Load strength shrinks and ends before x
            // The length of the load
            let ll = load.offset_end - load.offset_start;
            // The original triangular load from start to x
            let mut s_integral =
                load.strength / ll * x_dir_factor * (x - load.offset_start).powi(3) * 1.0 / 6.0;
            // The imaginary load after triangular loads end point (this is also triangular)
            s_integral +=
                load.strength / ll * x_dir_factor * (x - load.offset_end).powi(3) * 1.0 / 6.0;
            // The imaginary line load with the strength of triangular load which would go from loads start to x
            s_integral -= load.strength / ll * x_dir_factor * (x - load.offset_start).powi(3) / 3.0;
            // The imaginary line load with the strength of triangular load
            s_integral += load.strength * x_dir_factor * (x - load.offset_start).powi(2) / 2.0;
            return s_integral;
        } else {
            // Load strength shrinks and ends after x (needs to be split into triangular and line loads)
            // Split the load into a line load and a triangular load at x.
            let ll = load.offset_end - load.offset_start;
            // Intgeral of the axial force from triangular load
            let mut s_integral =
                load.strength / ll * x_dir_factor * (x - load.offset_start).powi(3) * 1.0 / 6.0;
            // The imaginary line load with the strength of smaller triangular load
            s_integral -= load.strength / ll * x_dir_factor * (x - load.offset_start).powi(3) / 3.0;
            // The imaginary line load with the strength of triangular load
            s_integral += load.strength * x_dir_factor * (x - load.offset_start).powi(2) / 2.0;

            return s_integral;
        }
    }
    0.0
}

/// Calculates the integral of the axial force function at x for a triangular load with the maximum load at the right hand side.
/// ltr = Left to right
fn handle_triang_rtl(load: &CalculationLoad, x: f64) -> f64 {
    let x_dir_factor = load.rotation.to_radians().cos();
    // Load offsets at left or right hand side
    let left = load.offset_end;
    let right = load.offset_start;
    if left <= x {
        if right <= x {
            let ll = right - left;
            let mut s_integral = load.strength / ll * x_dir_factor * (x - left).powi(3) / 6.0;
            s_integral -= load.strength * x_dir_factor * (x - right).powi(2) / 2.0;
            s_integral -= load.strength / ll * x_dir_factor * (x - right).powi(3) / 6.0;
            return s_integral;
        } else {
            // Split the load at x. No need to split into a line load, because of
            let ll = right - left;
            let s_integral = load.strength / ll * x_dir_factor * (x - left).powi(3) / 6.0;
            return s_integral;
        }
    }
    0.0
}
