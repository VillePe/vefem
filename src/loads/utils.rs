use std::collections::BTreeMap;

use vputilslib::equation_handler::EquationHandler;

use crate::fem::CalcModel;
use crate::loads::load::Load;
use crate::loads::load_combination::CalcLoadCombination;
use crate::structure::CalculationElement;

use super::load::CalculationLoad;
use super::{lc_utils, LoadGroup};

/// Gets the element numbers that are linked to given load. Different elements are separated with , (comma).
///
/// For multiple elements a 'S..E' (double dots with numbers before and after it) can be used.
///
/// If all load should be linked to all elements, -1 can be used (e.g. for self weight load)
pub fn get_linked_element_numbers(load: &Load) -> Vec<i32> {
    let split = load.element_numbers.split(",");
    let mut result: Vec<i32> = Vec::new();

    for s in split {
        if s.is_empty() {
            continue;
        }
        if s.contains("..") {
            // Split the S..E into S and E and collect it into a vector
            let range_split: Vec<&str> = s.split("..").collect();
            // If there are not exactly two objects, invalidate the value and continue.
            if range_split.len() != 2 {
                continue;
            }
            let num_begin: i32 = vputilslib::vputils::s_to_int(range_split[0]).unwrap() as i32;
            let num_end: i32 = vputilslib::vputils::s_to_int(range_split[1]).unwrap() as i32;
            for i in num_begin..=num_end {
                result.push(i);
            }
        } else {
            // Parse the numbers if there are no '..' modifier in the given string
            let num: i32 = vputilslib::vputils::s_to_int(s).unwrap() as i32;
            result.push(num);
        }
    }
    result
}

/// Checks if the given load is linked to given element by comparing the elements number to 'element_numbers' in [`Load`]
pub fn load_is_linked(elem_number: i32, linked_elem_numbers: &Vec<i32>) -> bool {
    linked_elem_numbers.contains(&-1) || linked_elem_numbers.contains(&elem_number)
}

/// Splits the trapezoid load into line load and triangular load. The first item in tuple is the
/// line load and the second item is the triangular load.
pub fn split_trapezoid_load(load: &Load, equation_handler: &EquationHandler) -> (Load, Load) {
    let split: Vec<&str> = load.strength.split(';').collect();
    if split.len() == 2 || split.len() == 1 {        
        let start_strength = equation_handler.calculate_formula(split[0]).unwrap_or(0.0);
        let end_strength = equation_handler.calculate_formula(split[1]).unwrap_or(0.0);
        return split_trapezoid_load_with_strengths(load, start_strength, end_strength);
    } else {
        println!("Error while parsing strength of the trapezoid load. Use semicolon ';' to separate the start and end strengths")
    }
    split_trapezoid_load_with_strengths(load, 0.0, 0.0)
}

/// Splits the trapezoid load into line load and triangular load. The first item in tuple is the
/// line load and the second item is the triangular load.
pub fn split_trapezoid_load_with_strengths(
    load: &Load,
    start_strength: f64,
    end_strength: f64,
) -> (Load, Load) {
    if start_strength < 0.0 || end_strength < 0.0 {
        println!("Trapezoid load can't have negative values!");
    }
    let t_load_offset_start;
    let t_load_offset_end;
    let tl_strength;
    // Handle the direction of the triangular load
    let ll_strength = if start_strength > end_strength {
        t_load_offset_start = load.offset_start.clone();
        t_load_offset_end = load.offset_end.clone();
        tl_strength = start_strength - end_strength;
        start_strength - tl_strength
    } else {
        t_load_offset_start = load.offset_end.clone();
        t_load_offset_end = load.offset_start.clone();
        tl_strength = end_strength - start_strength;
        end_strength - tl_strength
    };
    let line_load = Load::new_line_load(
        load.name.clone(),
        load.element_numbers.clone(),
        load.offset_start.clone(),
        load.offset_end.clone(),
        ll_strength.to_string(),
        load.rotation,
        load.load_group.clone(),
    );
    let tri_load = Load::new_triangular_load(
        load.name.clone(),
        load.element_numbers.clone(),
        t_load_offset_start.clone(),
        t_load_offset_end.clone(),
        tl_strength.to_string(),
        load.rotation,
        LoadGroup::PERMANENT,
    );
    (line_load, tri_load)
}

/// Creates a map of loads by load names.
/// ### Arguments
/// * `loads` - List of loads
pub fn get_load_map(loads: &Vec<Load>) -> BTreeMap<String, Vec<&Load>> {
    let mut load_map: BTreeMap<String, Vec<&Load>> = BTreeMap::new();
    for load in loads {
        if load_map.contains_key(&load.name) {
            load_map.get_mut(&load.name).unwrap().push(load);
        } else {
            load_map.insert(load.name.clone(), vec![load]);
        }
    }
    load_map
}

/// Creates a map of loads by load names.
/// ### Arguments
/// * `loads` - List of loads
pub fn get_calc_load_map(loads: Vec<CalculationLoad>) -> BTreeMap<String, Vec<CalculationLoad>> {
    let mut load_map: BTreeMap<String, Vec<CalculationLoad>> = BTreeMap::new();
    for load in loads {
        if load_map.contains_key(&load.name) {
            load_map.get_mut(&load.name).unwrap().push(load);
        } else {
            load_map.insert(load.name.clone(), vec![load]);
        }
    }
    load_map
}

/// Extracts the calculation loads from given loads. Note that this method also converts the
/// load strengths by their type (kN => N, kN/m => N/mm, kNm => Nmm)
/// * `elements` - List of elements
/// * `nodes` - List of nodes
/// * `loads` - List of loads
/// * `eq_handler` - Equation handler with pre initialized variables. Variable 'L' is preserved for element length.
pub fn extract_calculation_loads(
    calc_model: &CalcModel,
    loads: &Vec<Load>,
    load_combination: &CalcLoadCombination,
    eq_handler: &EquationHandler,
) -> Vec<CalculationLoad> {
    let mut calc_loads: Vec<CalculationLoad> = Vec::new();
    let mut temp_eq_handler = eq_handler.clone();
    let lc_is_empty = load_combination.loads_n_factors.is_empty();
    for load in loads {
        let mut strength_factor = 1.0;
        // Check if the loads_n_factor contains any items. If that vector is empty, all loads
        // will be extracted as calculation loads. Otherwise the loads will be extracted by names
        // in the loads_n_factor vector
        if !lc_is_empty {
            // Check if the current load is in the loads_n_factor vector
            if lc_utils::calc_load_is_included(load_combination, &load.name) {
                if load_combination.loads_n_factors.contains_key(&load.name) {
                    strength_factor = load_combination.loads_n_factors[&load.name];
                } else if load_combination.loads_n_factors.contains_key("ALL") {
                    strength_factor = load_combination.loads_n_factors["ALL"];
                } else {
                    strength_factor = 1.0;
                }
            } else {
                // If current load is not in the loads_n_factor vector, skip it
                continue;
            }
        }
        let rotation = load.rotation;
        let linked_elem_numbers = get_linked_element_numbers(load);
        for element in calc_model.get_all_calc_elements() {
            if !load_is_linked(element.model_el_num, &linked_elem_numbers) {
                continue;
            }
            let name = load.name.clone();
            let element_number = element.calc_el_num;
            temp_eq_handler.set_variable("L", element.model_el_length);
            let offset_start = temp_eq_handler
                .calculate_formula(&load.offset_start)
                .unwrap_or(0.0);
            let offset_end = temp_eq_handler
                .calculate_formula(&load.offset_end)
                .unwrap_or(0.0);
            let strength = temp_eq_handler
                .calculate_formula(&load.strength)
                .unwrap_or(0.0);

            if load.load_type == super::load::LoadType::Trapezoid {
                if offset_start < offset_end {
                    if offset_end < element.offset_from_model_el
                        || offset_start > element.offset_from_model_el + element.length
                    {
                        continue;
                    }
                } else {
                    if offset_start < element.offset_from_model_el
                        || offset_end > element.offset_from_model_el + element.length
                    {
                        continue;
                    }
                }
            } else {
                if offset_end < element.offset_from_model_el
                    || offset_start > element.offset_from_model_el + element.length
                {
                    continue;
                }
            }

            match load.load_type {
                crate::loads::load::LoadType::Point => {
                    let calc_load = CalculationLoad {
                        name,
                        offset_start: offset_start - element.offset_from_model_el,
                        strength: strength * 1e3 * strength_factor, // kN => N
                        rotation,
                        element_number,
                        load_type: super::load::CalculationLoadType::Point,
                        offset_end: 0.0,
                    };
                    calc_loads.push(calc_load);
                }
                super::load::LoadType::Line => {
                    let calc_load = handle_line_load_extracting(
                        element,
                        name,
                        offset_start,
                        offset_end,
                        strength * strength_factor,
                        rotation,
                        element_number,
                    );
                    calc_loads.push(calc_load);
                }
                super::load::LoadType::Triangular => {
                    let (tr_load, line_load) = handle_triang_load_extracting(
                        element,
                        name,
                        offset_start,
                        offset_end,
                        strength * strength_factor,
                        rotation,
                        element_number,
                    );
                    calc_loads.push(tr_load);
                    if line_load.is_some() {
                        calc_loads.push(line_load.unwrap());
                    }
                }
                super::load::LoadType::Rotational => {
                    let calc_load = CalculationLoad {
                        name,
                        offset_start: offset_start - element.offset_from_model_el,
                        offset_end,
                        strength: strength * strength_factor * 1e6,
                        rotation,
                        element_number,
                        load_type: super::load::CalculationLoadType::Rotational,
                    };
                    calc_loads.push(calc_load);
                }
                super::load::LoadType::Trapezoid => {
                    let (ll, tl) =
                        crate::loads::utils::split_trapezoid_load(load, &temp_eq_handler);
                    let strength = temp_eq_handler
                        .calculate_formula(&ll.strength)
                        .unwrap_or(0.0);

                    println!("LineLoad: {:?}", ll);
                    println!("TriangularLoad: {:?}", tl);

                    let calc_ll_load = handle_line_load_extracting(
                        element,
                        name.clone(),
                        offset_start,
                        offset_end,
                        strength * strength_factor,
                        rotation,
                        element_number,
                    );
                    calc_loads.push(calc_ll_load);
                    let offset_start = temp_eq_handler
                        .calculate_formula(&tl.offset_start)
                        .unwrap_or(0.0);
                    let offset_end = temp_eq_handler
                        .calculate_formula(&tl.offset_end)
                        .unwrap_or(0.0);
                    let strength = temp_eq_handler
                        .calculate_formula(&tl.strength)
                        .unwrap_or(0.0);
                    let (tr_load, line_load) = handle_triang_load_extracting(
                        element,
                        name,
                        offset_start,
                        offset_end,
                        strength * strength_factor,
                        rotation,
                        element_number,
                    );
                    calc_loads.push(tr_load);
                    if line_load.is_some() {
                        calc_loads.push(line_load.unwrap());
                    }
                }
                super::load::LoadType::Strain => {
                    let calc_load = CalculationLoad {
                        name,
                        offset_start,
                        offset_end,
                        strength: strength * strength_factor * element.length
                            / element.model_el_length,
                        rotation,
                        element_number,
                        load_type: super::load::CalculationLoadType::Strain,
                    };
                    calc_loads.push(calc_load);
                }
                super::load::LoadType::Thermal => {
                    // Convert the thermal coefficient to strain load
                    let thermal_coefficient = crate::material::get_thermal_expansion_coefficient(
                        element.material.value(),
                    );
                    let displacement = strength * thermal_coefficient * element.length;
                    let calc_load = CalculationLoad {
                        name,
                        offset_start,
                        offset_end,
                        strength: displacement * strength_factor,
                        rotation,
                        element_number,
                        load_type: super::load::CalculationLoadType::Strain,
                    };
                    calc_loads.push(calc_load);
                }
            }
        }
    }

    calc_loads
}

fn handle_line_load_extracting(
    calc_element: &CalculationElement,
    load_name: String,
    load_offset_start: f64,
    load_offset_end: f64,
    load_original_strength: f64,
    rotation: f64,
    element_number: i32,
) -> CalculationLoad {
    let offset_start;
    let offset_end;
    if load_offset_start <= calc_element.offset_from_model_el {
        offset_start = 0.0;
    } else {
        offset_start = load_offset_start - calc_element.offset_from_model_el;
    }
    if load_offset_end >= calc_element.offset_from_model_el + calc_element.length {
        offset_end = calc_element.length;
    } else {
        offset_end = load_offset_end - calc_element.offset_from_model_el;
    }
    let calc_load = CalculationLoad {
        name: load_name,
        offset_start,
        offset_end,
        strength: load_original_strength,
        rotation,
        element_number,
        load_type: super::load::CalculationLoadType::Line,
    };
    calc_load
}

fn handle_triang_load_extracting(
    calc_element: &CalculationElement,
    load_name: String,
    load_offset_start: f64,
    load_offset_end: f64,
    load_original_strength: f64,
    rotation: f64,
    element_number: i32,
) -> (CalculationLoad, Option<CalculationLoad>) {
    let mut tr_load = CalculationLoad {
        name: load_name.clone(),
        offset_start: 0.0, // This needs to be set in the ifs!
        offset_end: 0.0,   // This needs to be set in the ifs!
        strength: 0.0,     // This needs to be set in the ifs!
        rotation,
        element_number,
        load_type: super::load::CalculationLoadType::Triangular,
    };
    let mut line_load = CalculationLoad {
        name: load_name,
        offset_start: 0.0,
        offset_end: 0.0,
        strength: 0.0,
        load_type: super::load::CalculationLoadType::Line,
        ..tr_load
    };
    let original_load_length = (load_offset_start - load_offset_end).abs();
    let mut result_line_load: Option<CalculationLoad> = None;
    // Load is shrinking in positive X
    if load_offset_start < load_offset_end {
        // Load starts before the element
        if load_offset_start < calc_element.offset_from_model_el {
            // And load ends after the element
            if load_offset_end > calc_element.offset_from_model_el + calc_element.length {
                // The distance from start of the element to start of the load
                let x1 = calc_element.offset_from_model_el - load_offset_start;
                // The distance from end of the element to start of the load
                let x2 =
                    (calc_element.offset_from_model_el + calc_element.length) - load_offset_start;
                let strength_start = load_original_strength * (1.0 - x1 / original_load_length);
                let strength_end = load_original_strength * (1.0 - x2 / original_load_length);
                tr_load.strength = strength_start - strength_end;
                line_load.strength = strength_end;
                tr_load.offset_start = 0.0;
                tr_load.offset_end = calc_element.length;
                line_load.offset_start = 0.0;
                line_load.offset_end = calc_element.length;
                result_line_load = Some(line_load);
            } else {
                // Load ends before element (only need a single triangle load)
                let x1 = calc_element.offset_from_model_el - load_offset_start;
                let strength_start = load_original_strength * (1.0 - x1 / original_load_length);
                tr_load.strength = strength_start;
                tr_load.offset_start = 0.0;
                tr_load.offset_end = load_offset_end - calc_element.offset_from_model_el;
            }
        } else {
            // Load starts after the element
            // And load ends after the element
            if load_offset_end > calc_element.offset_from_model_el + calc_element.length {
                let x2 =
                    (calc_element.offset_from_model_el + calc_element.length) - load_offset_start;
                let strength_left = load_original_strength;
                let strength_right = load_original_strength * (1.0 - x2 / original_load_length);
                tr_load.strength = strength_left - strength_right;
                line_load.strength = strength_right;
                tr_load.offset_start = load_offset_start - calc_element.offset_from_model_el;
                tr_load.offset_end = calc_element.length;
                line_load.offset_start = load_offset_start - calc_element.offset_from_model_el;
                line_load.offset_end = calc_element.length;
                result_line_load = Some(line_load);
            } else {
                // Load ends before element (only need a single triangle load). The original
                // strength is correct
                tr_load.offset_start = load_offset_start - calc_element.offset_from_model_el;
                tr_load.offset_end = load_offset_end - calc_element.offset_from_model_el;
                tr_load.strength = load_original_strength;
            }
        }
    } else {
        // Note that the load_offset_start is the right end point and vice versa
        // Load starts before the element
        if load_offset_end < calc_element.offset_from_model_el {
            // And load ends after the element
            if load_offset_start > calc_element.offset_from_model_el + calc_element.length {
                // The distance from start of the element to left side of the load
                let x1 = calc_element.offset_from_model_el - load_offset_end;
                // The distance from end of the element to left side of the load
                let x2 =
                    (calc_element.offset_from_model_el + calc_element.length) - load_offset_end;
                let strength_left = load_original_strength * (x1 / original_load_length);
                let strength_right = load_original_strength * (x2 / original_load_length);
                tr_load.strength = strength_right - strength_left;
                line_load.strength = strength_left;
                tr_load.offset_end = 0.0;
                tr_load.offset_start = calc_element.length;
                line_load.offset_start = 0.0;
                line_load.offset_end = calc_element.length;
                result_line_load = Some(line_load);
            } else {
                // Load ends before element
                let x1 = calc_element.offset_from_model_el - load_offset_end;
                let strength_left = load_original_strength * (x1 / original_load_length);
                let strength_right = load_original_strength;
                tr_load.strength = strength_right - strength_left;
                line_load.strength = strength_left;
                tr_load.offset_end = 0.0;
                tr_load.offset_start = load_offset_start - calc_element.offset_from_model_el;
                line_load.offset_start = 0.0;
                line_load.offset_end = load_offset_start - calc_element.offset_from_model_el;
                result_line_load = Some(line_load);
            }
        } else {
            // Load starts after the element
            // And load ends after the element
            if load_offset_start > calc_element.offset_from_model_el + calc_element.length {
                let x2 =
                    (calc_element.offset_from_model_el + calc_element.length) - load_offset_end;
                let strength_left = 0.0;
                let strength_right = load_original_strength * (x2 / original_load_length);
                tr_load.strength = strength_right - strength_left;
                tr_load.offset_end = load_offset_end - calc_element.offset_from_model_el;
                tr_load.offset_start = calc_element.length;
            } else {
                // Load ends before element (only needed single triangle load). The original
                // strength is correct
                tr_load.offset_start = load_offset_start - calc_element.offset_from_model_el;
                tr_load.offset_end = load_offset_end - calc_element.offset_from_model_el;
                tr_load.strength = load_original_strength;
            }
        }
    }

    (tr_load, result_line_load)
}

#[cfg(test)]
mod tests {
    use crate::{material::Steel, profile::PolygonProfile, structure::Release};

    use super::*;

    #[test]
    fn t_get_element_list() {
        let load1 = Load {
            element_numbers: "1,2,3".to_string(),
            ..Load::default()
        };
        let result1 = get_linked_element_numbers(&load1);
        assert_eq!(vec![1, 2, 3], result1);

        let load2 = Load {
            element_numbers: "1,2,6,8".to_string(),
            ..Load::default()
        };
        let result2 = get_linked_element_numbers(&load2);
        assert_eq!(vec![1, 2, 6, 8], result2);

        let load3 = Load {
            element_numbers: "1,3..6,8".to_string(),
            ..Load::default()
        };
        let result3 = get_linked_element_numbers(&load3);
        assert_eq!(vec![1, 3, 4, 5, 6, 8], result3);

        let load4 = Load {
            element_numbers: "-1".to_string(),
            ..Load::default()
        };
        let result4 = get_linked_element_numbers(&load4);
        assert_eq!(vec![-1], result4);
    }

    #[test]
    fn t_handle_triang_load_extracting() {
        let calc_elem: CalculationElement = CalculationElement {
            calc_el_num: 1002,
            model_el_num: 1,
            model_el_length: 4000.0,
            node_start: 3,
            node_end: 4,
            material: &crate::material::MaterialData::Steel(Steel::new_s355()),
            profile: &crate::profile::Profile::PolygonProfile(PolygonProfile::new_rectangle(
                "name".to_string(),
                100.0,
                100.0,
            )),
            releases: Release::new(),
            length: 1000.0,
            rotation: 0.0,
            profile_area: 100.0 * 100.0,
            elastic_modulus: 210e3,
            major_smoa: 100.0 * 100.0f64.powi(3) / 12.0,
            offset_from_model_el: 1000.0,
        };
        let tr_load = Load::new_triangular_load(
            "ABC".to_string(),
            "1".to_string(),
            "XXX".to_string(),
            "XXX".to_string(),
            "XXX".to_string(),
            -90.0,
            LoadGroup::PERMANENT,
        );
        // Load starts before and ends after element. Shrinking from left to right
        let (tr, ll) = handle_triang_load_extracting(
            &calc_elem,
            tr_load.name.clone(),
            0.0,
            4000.0,
            10.0,
            -90.0,
            1,
        );
        println!("tr.strength: {0}", tr.strength);
        println!("tr.offset_start: {0}", tr.offset_start);
        println!("tr.offset_end: {0}", tr.offset_end);
        println!("ll.strength: {0}", ll.as_ref().unwrap().strength);
        println!("ll.offset_start: {0}", ll.as_ref().unwrap().offset_start);
        println!("ll.offset_end: {0}", ll.as_ref().unwrap().offset_end);
        println!();
        assert!(tr.strength == 2.5);
        assert!(ll.as_ref().unwrap().strength == 5.0);
        assert!(tr.offset_start == 0.0);
        assert!(tr.offset_end == 1000.0);
        assert!(ll.as_ref().unwrap().offset_start == 0.0);
        assert!(ll.as_ref().unwrap().offset_end == 1000.0);

        // Load starts before and ends before element. Shrinking from left to right
        let (tr, ll) = handle_triang_load_extracting(
            &calc_elem,
            tr_load.name.clone(),
            500.0,
            1500.0,
            10.0,
            -90.0,
            1,
        );
        println!("tr.strength: {0}", tr.strength);
        println!("tr.offset_start: {0}", tr.offset_start);
        println!("tr.offset_end: {0}", tr.offset_end);
        println!();
        assert!(tr.strength == 5.0);
        assert!(ll.is_none());
        assert!(tr.offset_start == 0.0);
        assert!(tr.offset_end == 500.0);

        // Load starts after and ends after element. Shrinking from left to right
        let (tr, ll) = handle_triang_load_extracting(
            &calc_elem,
            tr_load.name.clone(),
            1500.0,
            4000.0,
            10.0,
            -90.0,
            1,
        );
        println!("tr.strength: {0}", tr.strength);
        println!("tr.offset_start: {0}", tr.offset_start);
        println!("tr.offset_end: {0}", tr.offset_end);
        println!("ll.strength: {0}", ll.as_ref().unwrap().strength);
        println!("ll.offset_start: {0}", ll.as_ref().unwrap().offset_start);
        println!("ll.offset_end: {0}", ll.as_ref().unwrap().offset_end);
        println!();
        assert!(tr.strength == 2.0);
        assert!(ll.as_ref().unwrap().strength == 8.0);
        assert!(tr.offset_start == 500.0);
        assert!(tr.offset_end == 1000.0);
        assert!(ll.as_ref().unwrap().offset_start == 500.0);
        assert!(ll.as_ref().unwrap().offset_end == 1000.0);

        // Load starts after and ends before element. Shrinking from left to right
        let (tr, ll) = handle_triang_load_extracting(
            &calc_elem,
            tr_load.name.clone(),
            1200.0,
            1800.0,
            10.0,
            -90.0,
            1,
        );
        println!("tr.strength: {0}", tr.strength);
        println!("tr.offset_start: {0}", tr.offset_start);
        println!("tr.offset_end: {0}", tr.offset_end);
        println!();
        assert!(tr.strength == 10.0);
        assert!(ll.is_none());
        assert!(tr.offset_start == 200.0);
        assert!(tr.offset_end == 800.0);

        let tr_load = Load::new_triangular_load(
            "ABC".to_string(),
            "1".to_string(),
            "L".to_string(),
            "0".to_string(),
            "10".to_string(),
            -90.0,
            LoadGroup::PERMANENT,
        );
        // Load starts before and ends after element. Growing from left to right
        let (tr, ll) = handle_triang_load_extracting(
            &calc_elem,
            tr_load.name.clone(),
            4000.0,
            0.0,
            10.0,
            -90.0,
            1,
        );
        println!("tr.strength: {0}", tr.strength);
        println!("tr.offset_start: {0}", tr.offset_start);
        println!("tr.offset_end: {0}", tr.offset_end);
        println!("ll.strength: {0}", ll.as_ref().unwrap().strength);
        println!("ll.offset_start: {0}", ll.as_ref().unwrap().offset_start);
        println!("ll.offset_end: {0}", ll.as_ref().unwrap().offset_end);
        println!();
        assert!(tr.strength == 2.5);
        assert!(ll.as_ref().unwrap().strength == 2.5);
        assert!(tr.offset_start == 1000.0);
        assert!(tr.offset_end == 0000.0);
        assert!(ll.as_ref().unwrap().offset_start == 000.0);
        assert!(ll.as_ref().unwrap().offset_end == 1000.0);

        // Load starts before and ends before element. Growing from left to right
        let (tr, ll) = handle_triang_load_extracting(
            &calc_elem,
            tr_load.name.clone(),
            1500.0,
            0.0,
            10.0,
            -90.0,
            1,
        );
        println!("tr.strength: {0}", tr.strength);
        println!("tr.offset_start: {0}", tr.offset_start);
        println!("tr.offset_end: {0}", tr.offset_end);
        println!("ll.strength: {0}", ll.as_ref().unwrap().strength);
        println!("ll.offset_start: {0}", ll.as_ref().unwrap().offset_start);
        println!("ll.offset_end: {0}", ll.as_ref().unwrap().offset_end);
        println!();
        assert!((tr.strength - 3.333333).abs() < 0.1);
        assert!((ll.as_ref().unwrap().strength - 6.66666).abs() < 0.1);
        assert!(tr.offset_start == 500.0);
        assert!(tr.offset_end == 0000.0);
        assert!(ll.as_ref().unwrap().offset_start == 000.0);
        assert!(ll.as_ref().unwrap().offset_end == 500.0);

        // Load starts after and ends after element. Growing from left to right
        let (tr, ll) = handle_triang_load_extracting(
            &calc_elem,
            tr_load.name.clone(),
            4000.0,
            1500.0,
            10.0,
            -90.0,
            1,
        );
        println!("tr.strength: {0}", tr.strength);
        println!("tr.offset_start: {0}", tr.offset_start);
        println!("tr.offset_end: {0}", tr.offset_end);
        println!();
        assert!(tr.strength == 2.0);
        assert!(ll.is_none());
        assert!(tr.offset_start == 1000.0);
        assert!(tr.offset_end == 500.0);

        // Load starts after and ends before element. Growing from left to right
        let (tr, ll) = handle_triang_load_extracting(
            &calc_elem,
            tr_load.name.clone(),
            1800.0,
            1200.0,
            10.0,
            -90.0,
            1,
        );
        println!("tr.strength: {0}", tr.strength);
        println!("tr.offset_start: {0}", tr.offset_start);
        println!("tr.offset_end: {0}", tr.offset_end);
        println!();
        assert!(tr.strength == 10.0);
        assert!(ll.is_none());
        assert!(tr.offset_start == 800.0);
        assert!(tr.offset_end == 200.0);
    }
}
