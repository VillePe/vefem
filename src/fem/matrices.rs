#![allow(dead_code)]

use crate::structure::{CalculationElement, Node};
use nalgebra::DMatrix;
use std::collections::BTreeMap;

/// Gets the rotation matrix for the element. This matrix is in elements local coordinate system
pub fn get_element_rotation_matrix(element: &CalculationElement) -> DMatrix<f64> {
    let angle_radians = element.rotation.to_radians();
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
