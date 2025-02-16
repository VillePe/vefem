#![allow(dead_code)]

use crate::fem::equivalent_loads::create_joined_equivalent_loads;
use crate::fem::stiffness::create_joined_stiffness_matrix;
use crate::loads::Load;
use crate::structure::{Element, Node};
use nalgebra::DMatrix;
use std::collections::HashMap;
use vputilslib::equation_handler::EquationHandler;

use crate::fem::matrices::{
    get_unknown_translation_eq_loads_rows, get_unknown_translation_rows,
    get_unknown_translation_stiffness_rows,
};

pub fn calculate(
    elements: &Vec<Element>,
    nodes: &HashMap<i32, Node>,
    loads: &Vec<Load>,
    equation_handler: &mut EquationHandler,
) {
    let col_height = crate::fem::utils::col_height(nodes, elements);

    let global_stiff_matrix = create_joined_stiffness_matrix(elements, nodes);
    let global_equivalent_loads_matrix =
        create_joined_equivalent_loads(elements, nodes, loads, equation_handler);
    let displacements = calculate_displacements(
        nodes,
        col_height,
        &global_stiff_matrix,
        &global_equivalent_loads_matrix,
    );
    let reactions = calculate_reactions(&global_stiff_matrix, &displacements, &global_equivalent_loads_matrix);
}

/// Calculates the displacement matrix for given elements, nodes and loads. The displacement matrix is in global coordinates
pub fn calculate_displacements(
    nodes: &HashMap<i32, Node>,
    col_height: usize,
    global_stiff_matrix: &DMatrix<f64>,
    global_equivalent_loads_matrix: &DMatrix<f64>,
) -> DMatrix<f64> {
    // Get the rows with unknown translations to calculate the displacements for them.
    let unknown_translation_rows = get_unknown_translation_rows(nodes, &global_stiff_matrix);
    let unknown_translation_stiffness_rows =
        get_unknown_translation_stiffness_rows(&unknown_translation_rows, &global_stiff_matrix);
    let unknown_eq_loads_rows = get_unknown_translation_eq_loads_rows(
        &unknown_translation_rows,
        &global_equivalent_loads_matrix,
    );

    let stiffness_matrix_inverted = unknown_translation_stiffness_rows.try_inverse();
    let displacement = if let Some(inverted) = stiffness_matrix_inverted {
        inverted * unknown_eq_loads_rows
    } else {
        DMatrix::zeros(col_height, 1)
    };
    // Create the full displacement matrix by adding the calculated displacements to the unknown displacements (other rows are zero)
    let mut full_displacement_matrix: DMatrix<f64> = DMatrix::zeros(col_height, 1);
    for i in 0..unknown_translation_rows.len() {
        full_displacement_matrix[(unknown_translation_rows[i] as usize, 0)] = displacement[(i, 0)];
    }

    full_displacement_matrix
}

pub fn calculate_reactions(
    global_stiff_matrix: &DMatrix<f64>,
    global_displacement_matrix: &DMatrix<f64>,
    global_equivalent_loads_matrix: &DMatrix<f64>,
) -> DMatrix<f64> {
    global_stiff_matrix * global_displacement_matrix - global_equivalent_loads_matrix
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
