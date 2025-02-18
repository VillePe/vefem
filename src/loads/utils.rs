use std::collections::HashMap;

use vputilslib::equation_handler::{EquationHandler};

use crate::loads::load::Load;
use crate::structure::{element, Element, Node};

use super::load::CalculationLoad;

/// Gets the element numbers that are linked to given load. Different elements are separated with , (comma).
/// 
/// For multiple elements a 'S..E' (double dots with numbers before and after it) can be used.
///
/// If all load should be linked to all elements, -1 can be used (e.g. for self weight load) 
pub fn get_linked_element_numbers(load: &Load) -> Vec<i32> {
    let split = load.element_numbers.split(",");
    let mut result : Vec<i32> = Vec::new();

    for s in split {
        if s.is_empty() { continue; }
        if s.contains("..") {
            // Split the S..E into S and E and collect it into a vector
            let range_split : Vec<&str> = s.split("..").collect();
            // If there are not exactly two objects, invalidate the value and continue.
            if range_split.len() != 2 { continue; }
            let num_begin : i32 = vputilslib::vputils::s_to_int(range_split[0]).unwrap() as i32;
            let num_end : i32 = vputilslib::vputils::s_to_int(range_split[1]).unwrap() as i32;
            for i in num_begin..=num_end {
                result.push(i);
            }
        } else {
            // Parse the numbers if there are no '..' modifier in the given string
            let num : i32 = vputilslib::vputils::s_to_int(s).unwrap() as i32;
            result.push(num);
        }
    }
    result
}

/// Checks if the given load is linked to given element by comparing the elements number to 'element_numbers' in [`Load`]
pub fn load_is_linked(elem: &Element, load: &Load) -> bool {
    let linked_elements = get_linked_element_numbers(&load);
    linked_elements.contains(&-1) || linked_elements.contains(&elem.number)
}

/// Splits the trapezoid load into line load and triangular load. The first item in tuple is the 
/// line load and the second item is the triangular load.
pub fn split_trapezoid_load(load: &Load, equation_handler: &EquationHandler) -> (Load, Load) {
    let split: Vec<&str> = load.strength.split(';').collect();
    if split.len() == 2 || split.len() == 1 {
        let start_strength = equation_handler.calculate_formula(split[0]).unwrap_or(0.0);
        let end_strength = equation_handler.calculate_formula(split[0]).unwrap_or(0.0);
        return split_trapezoid_load_with_strengths(load, start_strength, end_strength);
    } else {
        println!("Error while parsing strength of the trapezoid load. Use semicolon ';' to separate the start and end strengths")
    }
    split_trapezoid_load_with_strengths(load, 0.0, 0.0)
}

/// Splits the trapezoid load into line load and triangular load. The first item in tuple is the 
/// line load and the second item is the triangular load.
pub fn split_trapezoid_load_with_strengths(load: &Load, start_strength: f64, end_strength: f64) -> (Load, Load) {
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
        load.rotation
    );
    let tri_load = Load::new_triangular_load(
        load.name.clone(),
        load.element_numbers.clone(),
        t_load_offset_start.clone(),
        t_load_offset_end.clone(),
        tl_strength.to_string(),
        load.rotation
    );
    (line_load, tri_load)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_get_element_list() {
        let load1 = Load{element_numbers: "1,2,3".to_string(), ..Load::default()};
        let result1 = get_linked_element_numbers(&load1);
        assert_eq!(vec![1,2,3], result1);

        let load2 = Load{element_numbers: "1,2,6,8".to_string(), ..Load::default()};
        let result2 = get_linked_element_numbers(&load2);
        assert_eq!(vec![1,2,6,8], result2);

        let load3 = Load{element_numbers: "1,3..6,8".to_string(), ..Load::default()};
        let result3 = get_linked_element_numbers(&load3);
        assert_eq!(vec![1,3,4,5,6,8], result3);

        let load4 = Load{element_numbers: "-1".to_string(), ..Load::default()};
        let result4 = get_linked_element_numbers(&load4);
        assert_eq!(vec![-1], result4);
    }
}

/// Extracts the calculation loads from given loads.
/// * `elements` - List of elements
/// * `nodes` - List of nodes
/// * `loads` - List of loads
/// * `eq_handler` - Equation handler with pre initialized variables. Variable 'L' is preserved for element length.
pub fn extract_calculation_loads(elements: &Vec<Element>, nodes: &HashMap<i32, Node>, loads: &Vec<Load>, eq_handler: &EquationHandler
) -> Vec<CalculationLoad> {
    let mut calc_loads : Vec<CalculationLoad> = Vec::new();
    let mut temp_eq_handler = eq_handler.clone();
    for load in loads {
        let rotation = load.rotation;
        for element in elements {
            if !load_is_linked(&element, load) {
                continue;
            }
            let element_number = element.number;
            let el_length = element.get_length(nodes);
            temp_eq_handler.set_variable("L", el_length);
            let offset_start = temp_eq_handler.calculate_formula(&load.offset_start).unwrap_or(0.0);
            let offset_end = temp_eq_handler.calculate_formula(&load.offset_end).unwrap_or(0.0);
            let strength = temp_eq_handler.calculate_formula(&load.strength).unwrap_or(0.0);
            match load.load_type {
                crate::loads::load::LoadType::Point => {
                    let calc_load = CalculationLoad {
                        offset_start, 
                        strength, 
                        rotation, 
                        element_number, 
                        load_type: super::load::CalculationLoadType::Point, 
                        offset_end: 0.0 };
                    calc_loads.push(calc_load);  
                }
                super::load::LoadType::Line => {
                    let calc_load = CalculationLoad {
                        offset_start, 
                        offset_end,
                        strength,  
                        rotation, 
                        element_number, 
                        load_type: super::load::CalculationLoadType::Line,
                        };
                    calc_loads.push(calc_load);
                },
                super::load::LoadType::Triangular => {
                    let calc_load = CalculationLoad {
                        offset_start, 
                        offset_end,
                        strength,
                        rotation, 
                        element_number, 
                        load_type: super::load::CalculationLoadType::Triangular,
                        };
                    calc_loads.push(calc_load);
                },
                super::load::LoadType::Rotational => {
                    let calc_load = CalculationLoad {
                        offset_start, 
                        offset_end,
                        strength,
                        rotation, 
                        element_number, 
                        load_type: super::load::CalculationLoadType::Line,
                        };
                    calc_loads.push(calc_load);
                },
                super::load::LoadType::Trapezoid => {
                    let (ll, tl) = crate::loads::utils::split_trapezoid_load(load, &temp_eq_handler);
                    let strength = temp_eq_handler.calculate_formula(&ll.strength).unwrap_or(0.0);
                    let calc_ll_load = CalculationLoad {
                        offset_start, 
                        offset_end,
                        strength,
                        rotation, 
                        element_number, 
                        load_type: super::load::CalculationLoadType::Line,
                        };
                    calc_loads.push(calc_ll_load);
                    let offset_start = temp_eq_handler.calculate_formula(&tl.offset_start).unwrap_or(0.0);
                    let offset_end = temp_eq_handler.calculate_formula(&tl.offset_end).unwrap_or(0.0);
                    let strength = temp_eq_handler.calculate_formula(&tl.strength).unwrap_or(0.0);
                    let calc_tl_load = CalculationLoad {
                        offset_start, 
                        offset_end,
                        strength,
                        rotation, 
                        element_number, 
                        load_type: super::load::CalculationLoadType::Triangular,
                        };
                    calc_loads.push(calc_tl_load);
                },
                super::load::LoadType::Strain => {
                    let calc_load = CalculationLoad {
                        offset_start, 
                        offset_end,
                        strength,
                        rotation, 
                        element_number, 
                        load_type: super::load::CalculationLoadType::Strain,
                        };
                    calc_loads.push(calc_load);
                },
                super::load::LoadType::Thermal => {                    
                    // Convert the thermal coefficient to strain load
                    let thermal_coefficient =
                        crate::material::get_thermal_expansion_coefficient(&element.material);
                    let displacement = strength * thermal_coefficient * el_length;
                    let calc_load = CalculationLoad {
                        offset_start, 
                        offset_end,
                        strength: displacement,
                        rotation, 
                        element_number, 
                        load_type: super::load::CalculationLoadType::Strain,
                        };
                    calc_loads.push(calc_load);
                },
            }
        }
    }

    calc_loads
}