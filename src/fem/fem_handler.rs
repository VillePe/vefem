#![allow(dead_code)]

use crate::fem::equivalent_loads::create_joined_equivalent_loads;
use crate::fem::stiffness::create_joined_stiffness_matrix;
use crate::loads::{utils, Load};
use crate::structure::{Element, Node};
use nalgebra::DMatrix;
use std::collections::HashMap;
use vputilslib::equation_handler::EquationHandler;

use crate::fem::matrices::{
    get_unknown_translation_eq_loads_rows, get_unknown_translation_rows,
    get_unknown_translation_stiffness_rows,
};

use super::NodeResults;

/// Calculates the displacements, support reactions and element internal forces.
/// * 'elements' - list of elements to calculate
/// * 'nodes' - list of nodes for the elements.
/// * 'loads' - list of loads to calculate
/// * 'equation_handler' - equation handler that can contain custom variables set by the user. 
/// The 'L' variable is reserved for the length of the element.
pub fn calculate(
    elements: &Vec<Element>,
    nodes: &HashMap<i32, Node>,
    loads: &Vec<Load>,
    equation_handler: &mut EquationHandler,
) -> NodeResults {
    let col_height = crate::fem::utils::col_height(nodes, elements);

    let calculation_loads = utils::extract_calculation_loads(elements, nodes, loads, equation_handler);

    let global_stiff_matrix = create_joined_stiffness_matrix(elements, nodes);
    let global_equivalent_loads_matrix =
        create_joined_equivalent_loads(elements, nodes, loads, equation_handler);
    let displacements = calculate_displacements(
        nodes,
        col_height,
        &global_stiff_matrix,
        &global_equivalent_loads_matrix,
    );
    let reactions = calculate_reactions(
        &global_stiff_matrix,
        &displacements,
        &global_equivalent_loads_matrix,
    );

    NodeResults::new(displacements, reactions, nodes.len(), &equation_handler)
}

/// Calculates the displacement matrix for given elements, nodes and loads. The displacement matrix
/// is in global coordinates.
/// To get the displacement for certain node, the corresponding row can be got with nodes
/// `number - 1 * dir` where
/// ```ignore
/// dir = 0|1|2
/// 0 = translation in X-axis
/// 1 = translation in Z-axis
/// 2 = rotation about Y-axis`.
/// ```
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
    // Create the full displacement matrix by adding the calculated displacements to the unknown
    // displacements (other rows are zero)
    let mut full_displacement_matrix: DMatrix<f64> = DMatrix::zeros(col_height, 1);
    for i in 0..unknown_translation_rows.len() {
        full_displacement_matrix[(unknown_translation_rows[i] as usize, 0)] = displacement[(i, 0)];
    }

    full_displacement_matrix
}

/// Calculates the support reaction matrix for given elements, nodes and loads. The reaction matrix
/// is in global coordinates. To get the support reaction for certain node, the corresponding row
/// can be got with nodes `number - 1 * dir` where
/// ```ignore
/// dir = 0|1|2
/// 0 = translation in X-axis
/// 1 = translation in Z-axis
/// 2 = rotation about Y-axis`.
/// ```
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
