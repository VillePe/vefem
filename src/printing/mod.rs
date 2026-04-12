use std::collections::{BTreeMap, HashMap};
use crate::loads::LoadCombination;
use crate::results::{CalculationResults, ForceType};
use crate::structure::{Element, Node, NodeCollection, StructureModel};

pub fn print_structure(structure_model: &StructureModel) {
    println!("ELEMENTS");
    for element in structure_model.elements.iter() {
        println!("{: <10}{: <10}{: <10}", element.number, element.node_start, element.node_end);
    }
    println!("NODES");
    for (_i, node) in structure_model.nodes.iter() {
        print!("{: <10}", node.number);
        print!("{: <20}", node.point.to_string());
        println!("{: <10}", node.support.to_short_string());
    }
}

pub fn print_results(results: &Vec<CalculationResults>, structure_model: &StructureModel, print_structure_flag: bool) {
    if print_structure_flag {
        print_structure(structure_model);
    }
    print_internal_forces(results, &structure_model.elements, &structure_model.nodes);
}

pub fn print_internal_forces(results: &Vec<CalculationResults>, elems: &Vec<Element>, nodes: &BTreeMap<i32, Node>) {
    for result in results {
        println!("Load combination: {}", result.load_combination);
        for elem in elems {
            print_element_internal_forces(result, elem, nodes);
        }
    }
}

macro_rules! write_internal_force {
    ($results:expr, $force_type:expr, $el_num:expr, $location:expr, $factor:expr) => {
        let force = $results.internal_force_results[&$el_num]
            .get_force_at($force_type, $location)
            .unwrap()
            .value_y;
        print!("{: >10.2}", force*$factor);
    };
}

pub fn print_element_internal_forces(results: &CalculationResults, element: &Element, nodes: &NodeCollection) {
    if !check_result_validity(results, element) {
        println!("WARN! Results are invalid! Printing canceled");
        println!("Element {}:", element.number);
        return;
    }

    let division = 10;
    let elem_length = element.get_length(nodes);
    let step = elem_length / (division as f64);
    let mut current_step = 0.0;
    println!("ELEMENT {}:", element.number);
    println!("MOMENTS");
    for _ in 0..division {
        write_internal_force!(results, ForceType::Moment, element.number, current_step, 1e-6);
        current_step += step;
    }
    write_internal_force!(results, ForceType::Moment, element.number, elem_length, 1e-6);
    println!();
    println!("SHEAR FORCES");
    current_step = 0.0;
    for _ in 0..division {
        write_internal_force!(results, ForceType::Shear, element.number, current_step, 1e-3);
        current_step += step;
    }
    write_internal_force!(results, ForceType::Shear, element.number, elem_length, 1e-3);
    println!();
    println!("AXIAL FORCES");
    current_step = 0.0;
    for _ in 0..division {
        write_internal_force!(results, ForceType::Axial, element.number, current_step, 1e-3);
        current_step += step;
    }
    write_internal_force!(results, ForceType::Axial, element.number, elem_length, 1e-3);
    println!();
    println!("DEFLECTION");
    current_step = 0.0;
    for _ in 0..division {
        write_internal_force!(results, ForceType::Deflection, element.number, current_step, 1.0);
        current_step += step;
    }
    write_internal_force!(results, ForceType::Deflection, element.number, elem_length, 1.0);
    println!();
}

fn check_result_validity(results: &CalculationResults, element: &Element) -> bool {
    let mut valid = true;
    if results.internal_force_results[&element.number].moment_forces.len() == 0 {
        println!("No moment forces for element {}", element.number);
        valid = false;
    }
    if results.internal_force_results[&element.number].shear_forces.len() == 0 {
        println!("No shear forces for element {}", element.number);
        valid = false;
    }
    if results.internal_force_results[&element.number].axial_forces.len() == 0 {
        println!("No axial forces for element {}", element.number);
        valid = false;
    }
    if results.internal_force_results[&element.number].deflections.len() == 0 {
        println!("No deflections for element {}", element.number);
        valid = false;
    }

    valid
}

/*
// 10 spaces between columns
ELEMENTS:
EL        1         100x100
NODES:
1         0.0       0.0       xxf
2         0.0       4000.0    fff
3         6000.0    0.0       fff
4         6000.0    4000.0    fff

LC 1
EL 1
MOMENT
//1         2         3         4       5       6       7       8       9       10      11
L0        L/10      2*L/10    ...
SHEAR
...
AXIAL FORCES
...
DEFLECTION
...
EL 2
...
EL 3
...
LC 2
...
LC 3
...
*/
