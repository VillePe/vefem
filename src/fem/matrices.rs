#![allow(dead_code)]

use crate::structure::Node;
use nalgebra::DMatrix;
use std::collections::BTreeMap;
use crate::fem::{equivalent_loads, matrices, CalcModel};
use crate::fem::stiffness::create_joined_stiffness_matrix;
use crate::loads::load::CalculationLoad;
use crate::settings::CalculationSettings;

pub struct CalculationMatrix {
    pub stiffness: DMatrix<f64>,
    pub equivalent_loads: DMatrix<f64>,
}

pub fn create_global_calculation_matrix(
    calc_model: &CalcModel, calc_settings: &CalculationSettings, calculation_loads: &Vec<CalculationLoad>
) -> CalculationMatrix {
    let mut global_stiff_matrix = create_joined_stiffness_matrix(calc_model, calc_settings);
    // The global equivalent loads matrix
    let mut global_eq_l_matrix = equivalent_loads::create(calc_model, calculation_loads, calc_settings);
    apply_support_rotation_values(calc_model.structure_nodes, &mut global_stiff_matrix, &mut global_eq_l_matrix);
    CalculationMatrix {
        stiffness: global_stiff_matrix,
        equivalent_loads: global_eq_l_matrix,
    }
}

/// Applies the rotations from supports to stiffness matrix and equivalent loads
fn apply_support_rotation_values(
    nodes: &BTreeMap<i32, Node>,
    global_stiff_matrix: &mut DMatrix<f64>,
    global_equivalent_loads_matrix: &mut DMatrix<f64>,
) {
    let dof = 3;
    for node in nodes.values() {
        if node.support.rotation != 0.0 && node.number > 0 {
            let node_number = node.number as usize;
            let small_rotation_matrix = matrices::get_small_rotation_matrix(node.support.rotation);
            let rotation_matrix_transposed = small_rotation_matrix.transpose();
            let mut small_stiff_matrix_col = DMatrix::zeros(nodes.len() * dof, dof);
            let mut small_stiff_matrix_row: DMatrix<f64> = DMatrix::zeros(dof, nodes.len() * dof);
            // Gather the columns (matrix size: nodes*dof, dof)
            for i in 0..nodes.len() * dof {
                for j in 0..dof {
                    small_stiff_matrix_col[(i, j)] = global_stiff_matrix
                        [(i, (node_number - 1) * dof + j)];
                }
            }
            // T*K*Ttranspose
            // K*Ttranspose
            let stiff_and_transposed = &small_stiff_matrix_col * rotation_matrix_transposed;
            // Update the global stiffness matrix.
            for i in 0..nodes.len() * dof {
                for j in 0..dof {
                    global_stiff_matrix[(i, (node_number - 1) * dof + j)] =
                        stiff_and_transposed[(i, j)];
                }
            }

            // Gather the columns (matrix size: dof, nodes*dof)
            for i in 0..dof {
                for j in 0..nodes.len() * dof {
                    small_stiff_matrix_row[(i, j)] = global_stiff_matrix
                        [((node_number - 1) * dof + i, j)];
                }
            }
            // T*KTtranspose = TKTtranspose
            let fully_rotated = &small_rotation_matrix * small_stiff_matrix_row;
            // Final update of the global stiffness matrix
            for i in 0..dof {
                for j in 0..nodes.len() *  dof {
                    global_stiff_matrix[((node_number - 1) * dof + i, j)] =
                        fully_rotated[(i, j)];
                }
            }
            let mut small_equivalent_loads_matrix = DMatrix::zeros(dof, 1);
            for i in 0..dof {
                small_equivalent_loads_matrix[(i, 0)] =
                    global_equivalent_loads_matrix[((node_number - 1) * dof + i, 0)]
            }
            // Rotate the equivalent loads matrix
            let rotated_equivalent_loads_matrix = &small_rotation_matrix * small_equivalent_loads_matrix;
            for i in 0..dof {
                global_equivalent_loads_matrix[((node_number - 1) * dof + i, 0)] =
                    rotated_equivalent_loads_matrix[(i, 0)];
            }
        }
    }
}

/// Gets the rotation matrix for the element. This matrix is in elements local coordinate system
pub fn get_rotation_matrix(rotation: f64) -> DMatrix<f64> {
    let angle_radians = rotation.to_radians();
    let c = angle_radians.cos();
    let s = angle_radians.sin();
    DMatrix::from_row_slice(
        6,
        6,
        &[
            c, s, 0.0, 0.0, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, c, s, 0.0, 0.0, 0.0, 0.0, -s, c, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
        //     c,   s,   0.0, 0.0, 0.0, 0.0,
        //     -s,  c,   0.0, 0.0, 0.0, 0.0,
        //     0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        //     0.0, 0.0, 0.0, c,   s,   0.0,
        //     0.0, 0.0, 0.0, -s,  c,   0.0,
        //     0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

/// Gets the rotation matrix for the node. This matrix is in elements local coordinate system
pub fn get_small_rotation_matrix(rotation: f64) -> DMatrix<f64> {
    let angle_radians = rotation.to_radians();
    let c = angle_radians.cos();
    let s = angle_radians.sin();
    DMatrix::from_row_slice(
        3,
        3,
        &[
            c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0,
        ],
        //     c,   s,   0.0,
        //     -s,  c,   0.0,
        //     0.0, 0.0, 1.0,
    )
}

pub fn get_unknown_translation_rows(nodes: &BTreeMap<i32, Node>, matrix: &DMatrix<f64>) -> Vec<i32> {
    let node_count = nodes.len();
    let mut result: Vec<i32> = Vec::new();
    let dof: usize = 3;
    for n in nodes {
        for i in 0..dof {
            // If the support translation is not locked -> row translation is unkown and add it to vector
            if !n.1.support.get_support_lock(i) {
                result.push((n.1.number - 1) * dof as i32 + i as i32);
            }
        }
    }
    // Gather the rows of element releases
    for i in (node_count * dof)..matrix.nrows() {
        result.push(i as i32);
    }

    // Iterating the hashmap does not guarantee the order of the nodes. Sort the result vector so that rows are in order
    result.sort();

    result
}

pub fn get_unknown_translation_stiffness_rows(
    unknown_translation_rows: &Vec<i32>,
    matrix: &DMatrix<f64>,
) -> DMatrix<f64> {
    let mut return_matrix = DMatrix::zeros(
        unknown_translation_rows.len(),
        unknown_translation_rows.len(),
    );
    let mut cur_row = 0;
    // Iterate through the unknown translation rows and columns and add them to the return matrix
    for row in 0..unknown_translation_rows.len() {
        let mut cur_col = 0;
        for col in 0..unknown_translation_rows.len() {
            return_matrix[(cur_row, cur_col)] = matrix[(
                unknown_translation_rows[row] as usize,
                unknown_translation_rows[col] as usize,
            )];
            cur_col += 1;
        }
        cur_row += 1;
    }

    return_matrix
}

pub fn get_unknown_translation_eq_loads_rows(
    unknown_translation_rows: &Vec<i32>,
    matrix: &DMatrix<f64>,
) -> DMatrix<f64> {
    let mut return_matrix = DMatrix::zeros(unknown_translation_rows.len(), 1);
    let mut cur_row = 0;
    // Iterate through the unknown translation rows and columns and add them to the return matrix
    for row in 0..unknown_translation_rows.len() {
        return_matrix[(cur_row, 0)] = matrix[(unknown_translation_rows[row] as usize, 0)];
        cur_row += 1;
    }

    return_matrix
}
