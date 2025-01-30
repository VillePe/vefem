#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::structure::element::Element;
use crate::structure::node::Node;
use nalgebra::DMatrix;
use std::collections::HashMap;

pub mod fem_handler;
pub mod matrices;

pub fn create_joined_stiffness_matrix(
    elements: Vec<&Element>,
    nodes: &HashMap<i32, Node>,
) -> DMatrix<f64> {
    let supp_count = nodes.len();
    // Increase the joined stiffness matrix size by release count. Releases are set into their
    // own rows and columns at the end of the joined matrix
    let release_count = crate::structure::utils::get_element_release_count(&elements);
    // The degrees of freedom count of single node (tx, tz, ry)
    let dof = 3;
    let row_width = (supp_count + release_count) * dof;

    let mut matrix_vector = DMatrix::from_row_slice(
        row_width,
        row_width,
        &vec![0.0; supp_count * supp_count * dof * dof],
    );


    // The starting row and column locations for locating the cells for releases
    let mut rel_row = supp_count * dof;

    for elem in elements {
        let e_glob_stiff_matrix = matrices::get_element_global_stiffness_matrix(&elem, nodes);
        // The index of the start node
        let s = (elem.node_start - 1) as usize;
        // The index of the end node
        let e = (elem.node_end - 1) as usize;
        for i in 0..dof*2 {
            // Reset the column counter at every row change
            let mut rel_col = supp_count * dof;
            for j in 0..dof*2 {
                if i < dof {
                    // If any of the start releases are set, the whole row must move to the rel_row
                    let start_any = elem.releases.start_release_any();
                    if j < dof {
                        // the top left triple (start element, start node)
                        if start_any && elem.releases.get_release_value(j).unwrap() {
                            // Place the value in the intersection of the current release row and column
                            matrix_vector[rel_row+rel_col] += e_glob_stiff_matrix[(i, j)];
                            // Move the current column with one so more releases can be set from 
                            // this row if there are any.
                            rel_col += 1;
                        } else if start_any {
                            // Place the value at the same column but move the row to the current release row
                            matrix_vector[(s * dof) * row_width + s * dof + j + rel_row] += e_glob_stiff_matrix[(i, j)];
                            rel_col += 1;
                        } else {
                            matrix_vector[(s * dof) * row_width + s * dof + j + i * row_width] += e_glob_stiff_matrix[(i, j)];
                        }
                    } else {
                        // The bottom left triple (start element, end node)
                        // Same as in if, but here the end release is used for column
                        if start_any && elem.releases.get_release_value(j).unwrap() {
                            matrix_vector[rel_row+rel_col] += e_glob_stiff_matrix[(i, j)];
                            rel_col += 1;
                        } else if start_any {
                            matrix_vector[(s * dof) * row_width + e * dof + (j - dof) + rel_row] += e_glob_stiff_matrix[(i, j)];
                            rel_col += 1;
                        } else {
                            matrix_vector[(s * dof) * row_width + e * dof + (j - dof) + i * row_width] += e_glob_stiff_matrix[(i, j)];
                        }
                    }
                } else {
                    // If any of the end releases are set, the whole row must move to the rel_row
                    let end_any = elem.releases.start_release_any();
                    if j < dof {
                        // the top right triple (end element, start node)
                        if end_any && elem.releases.get_release_value(j).unwrap() {
                            matrix_vector[rel_row+rel_col] += e_glob_stiff_matrix[(i, j)];
                            rel_col += 1;
                        } else if end_any {
                            matrix_vector[(e * dof) * row_width + s * dof + j + rel_row] += e_glob_stiff_matrix[(i, j)];
                            rel_col += 1;
                        } else {
                            matrix_vector[(e * dof) * row_width + s * dof + j + (i - dof) * row_width] += e_glob_stiff_matrix[(i, j)];
                        }
                    } else {
                        // the top right triple (end element, end node)
                        if end_any && elem.releases.get_release_value(j).unwrap() {
                            matrix_vector[rel_row+rel_col] += e_glob_stiff_matrix[(i, j)];
                            rel_col += 1;
                        } else if end_any {
                            matrix_vector[(e * dof) * row_width + e * dof + (j - dof) + rel_row] += e_glob_stiff_matrix[(i, j)];
                            rel_col += 1;
                        } else {
                            matrix_vector[(e * dof) * row_width + e * dof + (j - dof) + (i - dof) * row_width] += e_glob_stiff_matrix[(i, j)];
                        }
                        
                    }
                }
            }
            // Before moving to new row, increase the current row count by the number of releases
            if i < dof {
                rel_row += elem.releases.start_release_count();
            } else {
                rel_row += elem.releases.end_release_count();
            }
        }
    }

    matrix_vector
}
