use std::collections::HashMap;
use crate::loads::load::Load;
use crate::structure::{Element, Node};
use nalgebra::DMatrix;
use vputilslib::equation_handler::EquationHandler;
use crate::fem::matrices;

pub fn create_equivalent_loads(
    elements: &Vec<Element>,
    nodes: &HashMap<i32, Node>,
    loads: &Vec<&Load>,
    equation_handler: EquationHandler,
) -> DMatrix<f64> {
    for element in elements {
        let rot_matrix = matrices::get_element_rotation_matrix(&element, nodes);
    }
    
    DMatrix::from_row_slice(0, 0, &vec![0.0])
}
