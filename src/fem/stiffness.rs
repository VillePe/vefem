use std::collections::HashMap;
use nalgebra::DMatrix;
use crate::fem::matrices::get_element_rotation_matrix;
use crate::structure::element::{Element, MaterialType};
use crate::structure::Node;

/// Gets the elements stiffness matrix in the global coordinate system.
pub fn get_element_global_stiffness_matrix(e: &Element, nodes: &HashMap<i32, Node>) -> DMatrix<f64> {
    let e_stiff_matrix = get_element_stiffness_matrix(&e, nodes);
    let e_rotation_matrix = get_element_rotation_matrix(&e, nodes);
    let e_rot_matrix_T = e_rotation_matrix.transpose();
    let e_glob_stiff_matrix = e_rot_matrix_T * e_stiff_matrix * e_rotation_matrix;
    e_glob_stiff_matrix
}

/// Gets the stiffness matrix of the element in elements local coordinate system.
/// Do not use this directly in the calculations. Use get_element_global_stiffness_matrix
pub fn get_element_stiffness_matrix(element: &Element, nodes: &HashMap<i32, Node>) -> DMatrix<f64> {
    let E = match &element.material {
        MaterialType::Concrete(c) => {c.elastic_modulus}
        MaterialType::Steel(s) => {s.elastic_modulus}
        MaterialType::Timber(_) => {
            println!("Timber is not yet implemented!");
            0.0}
    };
    let L = element.get_length(nodes);
    let A = element.profile.get_area();
    let I = element.profile.get_major_second_mom_of_area();
    println!("{I}");
    let EA = E*A;
    let EI = E*I;
    DMatrix::from_row_slice(6,6, &[
        EA/L,  0.,                 0.,                -EA/L, 0.,                 0.,
        0.0,   12.0*EI/L.powi(3),  6.0*EI/L.powi(2),  0.0,   -12.0*EI/L.powi(3), 6.0*EI/L.powi(2),
        0.0,   6.0*EI/L.powi(2),   4.0*EI/L,          0.0,   -6.0*EI/L.powi(2),  2.0*EI/L,
        -EA/L, 0.0,                0.0,               EA/L,  0.0,                0.0,
        0.0,   -12.0*EI/L.powi(3), -6.0*EI/L.powi(2), 0.0,   12.0*EI/L.powi(3),  -6.0*EI/L.powi(2),
        0.0,   6.0*EI/L.powi(2),   2.0*EI/L,          0.0,   -6.0*EI/L.powi(2),  4.0*EI/L,
    ])
}

pub fn create_joined_stiffness_matrix(
    elements: &Vec<Element>,
    nodes: &HashMap<i32, Node>,
) -> DMatrix<f64> {
    let supp_count = nodes.len();
    // Increase the joined stiffness matrix size by release count. Releases are set into their
    // own rows and columns at the end of the joined matrix
    let release_count = crate::structure::utils::get_element_release_count(&elements);
    // The degrees of freedom count of single node (tx, tz, ry)
    let dof = 3;
    let row_width = supp_count * dof + release_count;

    let mut matrix_vector = vec![0.0; row_width * row_width];

    // The starting row and column locations for locating the cells for releases
    let mut rel_row = supp_count * dof;
    let mut rel_increment_count = 0;
    let mut supp_index1: usize;
    let mut supp_index2: usize;
    let mut i_normalized: usize;
    let mut j_normalized: usize;

    for elem in elements {
        let e_glob_stiff_matrix = get_element_global_stiffness_matrix(&elem, nodes);
        // The index of the start node
        let s = (elem.node_start - 1) as usize;
        // The index of the end node
        let e = (elem.node_end - 1) as usize;
        for i in 0..dof*2 {
            // Reset the column counter at every row change
            let mut rel_col = supp_count * dof + rel_increment_count;
            let mut increment_rel_row_count = false;
            for j in 0..dof*2 {
                if i < dof {
                    supp_index1 = s;
                    i_normalized = i;
                    if j < dof {
                        // the top left triple (start element, start node)
                        supp_index2 = s;
                        j_normalized = j;
                    } else {
                        // The bottom left triple (start element, end node)
                        supp_index2 = e;
                        j_normalized = j-dof;
                    }
                } else {
                    supp_index1 = e;
                    i_normalized = i-dof;
                    if j < dof {
                        // the top right triple (end element, start node)
                        supp_index2 = s;
                        j_normalized = j;
                    } else {
                        // the top right triple (end element, end node)
                        supp_index2 = e;
                        j_normalized = j-dof;
                    }
                }
                // If there is a release at either i or j, it needs to be handled
                if elem.releases.get_release_value(i).unwrap() || elem.releases.get_release_value(j).unwrap() {
                    if i == j {
                        // If current row and column have release, place the value in the intersection of the current
                        // release row and column
                        matrix_vector[rel_row * row_width + rel_col] += e_glob_stiff_matrix[(i, j)];
                        rel_col += 1;
                        rel_increment_count += 1;
                    } else if elem.releases.get_release_value(i).unwrap() {
                        // If the current row has a release, move the whole row to the rel_row
                        matrix_vector[supp_index2 * dof + j_normalized + rel_row * row_width]
                            += e_glob_stiff_matrix[(i, j)];
                        increment_rel_row_count = true;
                    } else if elem.releases.get_release_value(j).unwrap() {
                        // If the current column has a release, move the whole column to the rel_col
                        matrix_vector[(supp_index1 * dof) * row_width + i_normalized * row_width + rel_col]
                            += e_glob_stiff_matrix[(i, j)];
                        rel_col += 1;
                    }
                } else {
                    // (supp_index1 * dof) * row_width       offset the rows by the support node number
                    // supp_index2 * dof                     offset the columns by the support number
                    // j_normalized                          offset the columns by j
                    // i_normalized * row_width              offset the rows by i
                    matrix_vector[(supp_index1 * dof)*row_width+i_normalized*row_width+ supp_index2 * dof + j_normalized]
                        += e_glob_stiff_matrix[(i, j)];
                }
            }
            // Before moving to new row, increase the current row count by the number of releases
            if increment_rel_row_count {
                rel_row += 1;
                println!("Increment rel row count, rel_row: {}", rel_row);
            }
        }
    }

    DMatrix::from_row_slice(row_width, row_width, &matrix_vector)
}