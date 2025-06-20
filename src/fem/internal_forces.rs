#![allow(dead_code)]

use std::collections::BTreeMap;

use crate::{
    loads::load::{self, CalculationLoad},
    results::{ForceType, InternalForcePoint, InternalForceResults},
    settings::{self, CalculationSettings},
    structure::CalculationElement,
};

use crate::results::NodeResults;

use super::{axial_deformation, deflection, CalcModel};

/// Calculates the internal forces for the elements by support reactions and displacements in node results
///
/// Parameters:
/// * 'elements' - the elements of the structure model
/// * 'nodes' - the nodes of the structure model
/// * 'loads' - the loads of the calculation model
/// * 'node_results' - the node results of the structure
/// * 'calc_settings' - the calculation settings
/// Returns: BTreeMap<i32, InternalForceResults> where the key is the element number
pub fn calc_internal_forces(
    calc_model: &CalcModel,
    loads: &Vec<CalculationLoad>,
    node_results: &NodeResults,
    calc_settings: &CalculationSettings,
) -> BTreeMap<i32, InternalForceResults> {
    let mut map: BTreeMap<i32, InternalForceResults> = BTreeMap::new();
    for structure_element in calc_model.structure_elements {
        let mut moment_forces = vec![];
        let mut shear_forces = vec![];
        let mut axial_forces = vec![];
        let mut deflections = vec![];
        for element in calc_model.calc_elements[&structure_element.number].iter() {
            let element_length = element.length;
            let split_interval = match calc_settings.calc_split_interval {
                settings::calc_settings::CalcSplitInterval::Absolute(a) => a,
                settings::calc_settings::CalcSplitInterval::Relative(r) => element_length * r,
            };
            let mut x = 0.0;
            let mut last = false;
            while x < element_length || last {
                let moment_force_val =
                    calculate_moment_at(x, element, loads, node_results, calc_settings);
                let axial_force_val =
                    calculate_axial_force_at(x, element, loads, node_results, calc_settings);
                let shear_force_val =
                    calculate_shear_at(x, element, loads, node_results, calc_settings);
                let deflection_val =
                    deflection::calculate_at(x, element, loads, calc_settings, node_results);
                let axial_deformation_val =
                    axial_deformation::calculate_at(x, element, loads, node_results, calc_settings);

                moment_forces.push(InternalForcePoint {
                    force_type: ForceType::Moment,
                    value_x: 0.0,
                    value_y: moment_force_val,
                    pos_on_element: x + element.offset_from_model_el,
                    element_number: element.model_el_num,
                    load_comb_number: 0,
                });
                axial_forces.push(InternalForcePoint {
                    force_type: ForceType::Axial,
                    value_x: 0.0,
                    value_y: axial_force_val,
                    pos_on_element: x + element.offset_from_model_el,
                    element_number: element.model_el_num,
                    load_comb_number: 0,
                });
                shear_forces.push(InternalForcePoint {
                    force_type: ForceType::Shear,
                    value_x: 0.0,
                    value_y: shear_force_val,
                    pos_on_element: x + element.offset_from_model_el,
                    element_number: element.model_el_num,
                    load_comb_number: 0,
                });
                deflections.push(InternalForcePoint {
                    force_type: ForceType::Deflection,
                    value_x: axial_deformation_val,
                    value_y: deflection_val,
                    pos_on_element: x + element.offset_from_model_el,
                    element_number: element.model_el_num,
                    load_comb_number: 0,
                });

                x += split_interval;

                // Make sure that last point is exactly at the end of the element
                if last {
                    break;
                }
                if x >= element_length {
                    x = element_length;
                    last = true;
                }
            }
        }

        let res = InternalForceResults {
            element_number: structure_element.number,
            axial_forces,
            shear_forces,
            moment_forces,
            deflections,
        };
        map.insert(structure_element.number, res);
    }

    map
}

pub fn calculate_moment_at(
    x: f64,
    element: &CalculationElement,
    loads: &Vec<CalculationLoad>,
    results: &NodeResults,
    settings: &CalculationSettings,
) -> f64 {
    let mut moment = 0.0;
    let local_reactions = results.get_elem_local_nodal_force_vectors(element, loads, settings);
    for load in loads {
        if load.element_number != element.calc_el_num {
            continue;
        }
        // The factor to handle skewed loads
        let z_dir_factor = load.rotation.to_radians().sin();
        match load.load_type {
            load::CalculationLoadType::Point => {
                if load.offset_start <= x {
                    moment += load.strength * z_dir_factor * (x - load.offset_start);
                }
            }
            load::CalculationLoadType::Rotational => {
                if load.offset_start <= x {
                    moment -= load.strength;
                }
            }
            load::CalculationLoadType::Line => {
                if load.offset_start <= x {
                    let load_length;
                    if load.offset_end <= x {
                        load_length = load.offset_end - load.offset_start;
                    } else {
                        load_length = x - load.offset_start;
                    }
                    let offset = x - (load.offset_start + load_length / 2.0);
                    moment += load.strength * z_dir_factor * load_length * offset;
                }
            }
            load::CalculationLoadType::Triangular => {
                // Triangular load with max load at left hand side
                if load.offset_start < load.offset_end {
                    moment += moment_triang_ltr(load, x)
                } else {
                    moment += moment_triang_rtl(load, x)
                }
            }
            load::CalculationLoadType::Strain => {}
        };
    }

    moment += local_reactions[(1, 0)] * x - local_reactions[(2, 0)];

    moment
}

pub fn calculate_shear_at(
    x: f64,
    element: &CalculationElement,
    loads: &Vec<CalculationLoad>,
    results: &NodeResults,
    settings: &CalculationSettings,
) -> f64 {
    let mut shear = 0.0;
    let local_reactions = results.get_elem_local_nodal_force_vectors(element, loads, settings);
    for load in loads {
        if load.element_number != element.calc_el_num {
            continue;
        }
        // The factor to handle skewed loads
        let z_dir_factor = load.rotation.to_radians().sin();
        match load.load_type {
            load::CalculationLoadType::Point => {
                if load.offset_start <= x {
                    shear += load.strength * z_dir_factor;
                }
            }
            load::CalculationLoadType::Rotational => {}
            load::CalculationLoadType::Line => {
                if load.offset_start <= x {
                    let load_length;
                    if load.offset_end <= x {
                        load_length = load.offset_end - load.offset_start;
                    } else {
                        load_length = x - load.offset_start;
                    }
                    shear += load.strength * z_dir_factor * load_length;
                }
            }
            load::CalculationLoadType::Triangular => {
                // Triangular load with max load at left hand side
                if load.offset_start < load.offset_end {
                    shear += handle_linear_force_triang_ltr(load, x, z_dir_factor)
                } else {
                    shear += handle_linear_force_triang_rtl(load, x, z_dir_factor)
                }
            }
            load::CalculationLoadType::Strain => {}
        };
    }

    shear += local_reactions[(1, 0)];

    shear
}

pub fn calculate_axial_force_at(
    x: f64,
    element: &CalculationElement,
    loads: &Vec<CalculationLoad>,
    results: &NodeResults,
    settings: &CalculationSettings,
) -> f64 {
    let mut axial_f = 0.0;
    let local_reactions = results.get_elem_local_nodal_force_vectors(element, loads, settings);
    for load in loads {
        if load.element_number != element.calc_el_num {
            continue;
        }
        // The factor to handle skewed loads
        let x_dir_factor = -load.rotation.to_radians().cos();
        match load.load_type {
            load::CalculationLoadType::Point => {
                if load.offset_start <= x {
                    axial_f += load.strength * x_dir_factor;
                }
            }
            load::CalculationLoadType::Rotational => {}
            load::CalculationLoadType::Line => {
                if load.offset_start <= x {
                    let load_length;
                    if load.offset_end <= x {
                        load_length = load.offset_end - load.offset_start;
                    } else {
                        load_length = x - load.offset_start;
                    }
                    axial_f += load.strength * x_dir_factor * load_length;
                }
            }
            load::CalculationLoadType::Triangular => {
                // Triangular load with max load at left hand side
                if load.offset_start < load.offset_end {
                    axial_f += handle_linear_force_triang_ltr(load, x, x_dir_factor)
                } else {
                    axial_f += handle_linear_force_triang_rtl(load, x, x_dir_factor)
                }
            }
            load::CalculationLoadType::Strain => {}
        };
    }

    axial_f -= local_reactions[(0, 0)];

    axial_f
}

/// Calculates the moment at x for a triangular load with the maximum load at the left hand side.
/// ltr = Left to right
fn moment_triang_ltr(load: &CalculationLoad, x: f64) -> f64 {
    let mut moment = 0.0;
    let z_dir_factor = load.rotation.to_radians().sin();
    if load.offset_start <= x {
        if load.offset_end <= x {
            let load_length = load.offset_end - load.offset_start;
            let offset = x - (load.offset_start + load_length * 1.0 / 3.0);
            moment += load.strength * z_dir_factor * load_length / 2.0 * offset;
        } else {
            // Split the load into a line load and a triangular load at x.
            let load_length = x - load.offset_start;
            let offset_tl = x - (load.offset_start + (load_length) * 1.0 / 3.0);
            // The minimum strength at x (right hand side of the load)
            let strength_min = load.strength
                - load.strength * (x - load.offset_start) / (load.offset_end - load.offset_start);
            let strength_ll = strength_min;
            let strength_tl = load.strength - strength_ll;
            // Moment from triangular load = F * l / 2 * offset
            moment += strength_tl * z_dir_factor * load_length / 2.0 * offset_tl;
            let offset_ll = x - (load.offset_start + load_length / 2.0);
            moment += strength_ll * z_dir_factor * load_length * offset_ll;
        }
    }
    moment
}

/// Calculates the moment at x for a triangular load with the maximum load at the right hand side.
/// ltr = Left to right
fn moment_triang_rtl(load: &CalculationLoad, x: f64) -> f64 {
    let z_dir_factor = load.rotation.to_radians().sin();
    // Load offsets at left or right hand side
    let left = load.offset_end;
    let right = load.offset_start;
    if left <= x {
        let load_length;
        let offset;
        if right <= x {
            offset = x - (left + (right - left) * 2.0 / 3.0);
            load_length = right - left;
            return load.strength * z_dir_factor * load_length / 2.0 * offset;
        } else {
            // Split the load at x. No need to split into a line load, because of
            // the direction
            let load_length = x - left;
            let offset_tl = x - (left + (load_length) * 2.0 / 3.0);
            // The minimum strength at x (right hand side of the load)
            let strength_max = load.strength * (x - left) / (right - left);
            return strength_max * z_dir_factor * load_length / 2.0 * offset_tl;
        }
    }
    0.0
}

/// Calculates the shear or axial force at x for a triangular load with the maximum load at the left
/// hand side.
/// ltr = Left to right
/// * 'dir_factor' - the factor to handle the direction of the load. For shear use ``load.rotation.to_radians().sin()`` and for axial force use ``...cos()``)
fn handle_linear_force_triang_ltr(load: &CalculationLoad, x: f64, dir_factor: f64) -> f64 {
    let mut shear = 0.0;
    if load.offset_start <= x {
        if load.offset_end <= x {
            let load_length = load.offset_end - load.offset_start;
            shear += load.strength * dir_factor * load_length / 2.0;
        } else {
            // Split the load into a line load and a triangular load at x.
            let load_length = x - load.offset_start;
            // The minimum strength at x (right hand side of the load)
            let strength_min = load.strength
                - load.strength * (x - load.offset_start) / (load.offset_end - load.offset_start);
            let strength_ll = strength_min;
            let strength_tl = load.strength - strength_ll;
            // Moment from triangular load = F * l / 2 * offset
            shear += strength_tl * dir_factor * load_length / 2.0;
            shear += strength_ll * dir_factor * load_length;
        }
    }
    shear
}

/// Calculates the shear or axial force at x for a triangular load with the maximum load at the
/// right hand side.
/// ltr = Left to right
/// * 'dir_factor' - the factor to handle the direction of the load. For shear use ``load.rotation.to_radians().sin()`` and for axial force use ``...cos()``)
fn handle_linear_force_triang_rtl(load: &CalculationLoad, x: f64, dir_factor: f64) -> f64 {
    // Load offsets at left or right hand side
    let left = load.offset_end;
    let right = load.offset_start;
    if left <= x {
        let load_length;
        if right <= x {
            load_length = right - left;
            return load.strength * dir_factor * load_length / 2.0;
        } else {
            // Split the load at x. No need to split into a line load, because of
            // the direction
            let load_length = x - left;
            // The minimum strength at x (right hand side of the load)
            let strength_max = load.strength * (x - left) / (right - left);
            return strength_max * dir_factor * load_length / 2.0;
        }
    }
    0.0
}
