#![allow(dead_code)]

use std::collections::HashMap;
use nalgebra::DMatrix;
use vputilslib::equation_handler::EquationHandler;
use crate::fem::equivalent_loads::create_joined_equivalent_loads;
use crate::fem::stiffness::create_joined_stiffness_matrix;
use crate::loads::Load;
use crate::structure::{Element, Node};

pub fn calculate_displacements(
    elements: &Vec<Element>,
    nodes: &HashMap<i32, Node>,
    loads: &Vec<Load>,
    equation_handler: &mut EquationHandler,
) -> DMatrix<f64> {
    let supp_count = nodes.len();
    // Increase the joined stiffness matrix size by release count. Releases are set into their
    // own rows and columns at the end of the joined matrix
    let release_count = crate::structure::utils::get_element_release_count(&elements);
    // The degrees of freedom count of single node (tx, tz, ry)
    let dof = 3;
    let col_height = supp_count * dof + release_count;

    let mut matrix_vector = vec![0.0; col_height];
    
    let global_stiff_matrix = create_joined_stiffness_matrix(elements, nodes);
    let global_equivalent_loads_matrix = create_joined_equivalent_loads(elements, nodes, loads, equation_handler);
    

    DMatrix::from_row_slice(col_height, 1, &matrix_vector)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

    }
}