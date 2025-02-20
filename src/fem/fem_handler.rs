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

    let _calculation_loads = utils::extract_calculation_loads(elements, nodes, loads, equation_handler);

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
    let displacement : DMatrix<f64>;
    // If there are big number of rows with unknown translations, use cholesky decomposition for
    // solving the system of equations. Otherwise use regular inversion (might not be necessary, 
    // maybe could always solve with cholesky. Could be benchmarked).
    if unknown_translation_stiffness_rows.nrows() > 100 {
        displacement = displacements_cholesky(unknown_translation_stiffness_rows, &unknown_eq_loads_rows).unwrap_or(DMatrix::zeros(col_height, 1));
    } else {
        let stiffness_matrix_inverted = invert_stiff_matrix(unknown_translation_stiffness_rows);

        displacement = if let Some(inverted) = stiffness_matrix_inverted {
            inverted * unknown_eq_loads_rows
        } else {
            DMatrix::zeros(col_height, 1)
        };
    }    
    // Create the full displacement matrix by adding the calculated displacements to the unknown
    // displacements (other rows are zero)
    let mut full_displacement_matrix: DMatrix<f64> = DMatrix::zeros(col_height, 1);
    for i in 0..unknown_translation_rows.len() {
        full_displacement_matrix[(unknown_translation_rows[i] as usize, 0)] = displacement[(i, 0)];
    }

    full_displacement_matrix
}

/// Calculates the displacements using cholesky decomposition for given rows that have unknown translations
/// * 'unknown_translation_stiffness_rows' - stiffness matrix rows that have unknown translations
/// * 'unknown_eq_loads' - equivalent loads at the same rows as the stiffness matrix
fn displacements_cholesky(unknown_translation_stiffness_rows: DMatrix<f64>, unknown_eq_loads: &DMatrix<f64>) -> Option<DMatrix<f64>> {
    match unknown_translation_stiffness_rows.cholesky() {
        Some(cholesky) => {   
            Some(cholesky.solve(&unknown_eq_loads))
        },
        None => None,
    }
}

/// Creates the inverted stiffness matrix for given matrix. If the matrix is larger than 100x100,
/// cholesky decomposition is used for inversion. Otherwise regular inversion is used.
/// * 'matrix' - matrix to invert (should be the stiffness matrix with uknonwn translations)
fn invert_stiff_matrix(matrix: DMatrix<f64>) -> Option<DMatrix<f64>> {
    println!("Using regular inversion...");
    return matrix.try_inverse()
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
    use std::{collections::HashMap, time::SystemTime};

    use vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};

    use crate::{loads::Load, material::Steel, structure::{element::MaterialType, Element, Node, Profile}};

    use super::calculate;

    // #[test]
    fn t_simple_benchmark_calculation() {
        let mut elements : Vec<Element>  = vec![];
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        // Create multiple 4 meter long elements to test the speed of calculations
        for i in 0..1000 {
            nodes.insert(i+2, Node::new_hinged(i+2, VpPoint::new(((i+1) as f64)*4000.0f64, 0.0)));
            elements.push(Element::new(i+1, i+1, i+2, 
                Profile::new_rectangle("100x100".to_string(), 100.0, 100.0), 
                MaterialType::Steel(Steel::new(210e3))));
        }
        let timer = SystemTime::now();
        let load = Load::new_line_load("Lineload".to_string(), "-1".to_string(), "0".to_string(), "L".to_string(), "10".to_string(), -90.0);

        let results = calculate(&elements, &nodes, &vec![load], &mut EquationHandler::new());
        println!("Calculation time: {:?}", timer.elapsed().unwrap());
        println!("Element count: {}", elements.len());
        println!("Node count: {}", nodes.len());
        println!("Result displacement row count: {:?}", results.displacements.nrows());
        println!("Support reaction (0,1): {} kN", results.get_support_reaction(1, 1)/1000.0);
    }
    

}
