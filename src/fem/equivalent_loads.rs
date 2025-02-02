use std::collections::HashMap;
use crate::loads::load::Load;
use crate::structure::{Element, Node};
use nalgebra::{DMatrix, Point};
use vputilslib::equation_handler::EquationHandler;
use crate::fem::matrices;
use crate::loads;

pub fn get_joined_equivalent_loads(
    elements: &Vec<Element>,
    nodes: &HashMap<i32, Node>,
    loads: &Vec<&Load>,
    equation_handler: EquationHandler,
) -> DMatrix<f64> {
    let dof = 3;
    let matrix_vector : Vec<f64> = vec![0.0; nodes.len() * dof];
    
    for element in elements {
        let rot_matrix = matrices::get_element_rotation_matrix(&element, nodes);
    }
    
    DMatrix::from_row_slice(0, 0, &vec![0.0])
}

pub fn get_element_global_equivalent_loads(
    element: Element,
    loads: &Vec<&Load>,
    equation_handler: EquationHandler,
) -> DMatrix<f64> {
    let mut linked_loads: Vec<&Load> = Vec::new();
    // Gather the loads that are linked to the given element
    for l in loads {
        if loads::utils::load_is_linked(&element, &l) {
            linked_loads.push(l);
        }
    }
    // Iterate through the linked loads and add them to the equivalent load matrix
    for l in linked_loads {
        
    }

    DMatrix::from_row_slice(6, 6, &vec![0.0])
}

fn handle_point_load(load: &Load) -> DMatrix<f64> {
    
}