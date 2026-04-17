#![allow(non_snake_case)]

use std::collections::BTreeMap;
use crate::fem::matrices::get_rotation_matrix;
use crate::material::MaterialData;
use crate::settings::CalculationSettings;
use crate::structure::CalculationElement;
use nalgebra::DMatrix;
use crate::structure::element::ReleaseIndexMap;
use super::CalcModel;

/// Gets the elements stiffness matrix in the global coordinate system.
pub fn get_element_global_stiffness_matrix(
    e: &CalculationElement,
    settings: &CalculationSettings
) -> DMatrix<f64> {    
    let e_stiff_matrix = get_element_stiffness_matrix(&e, settings);
    let e_rotation_matrix = get_rotation_matrix(e.rotation);
    let e_rot_matrix_t = e_rotation_matrix.transpose();
    let e_glob_stiff_matrix = e_rot_matrix_t * e_stiff_matrix * e_rotation_matrix;
    e_glob_stiff_matrix
}

/// Gets the stiffness matrix of the element in elements local coordinate system.
/// Do not use this directly in the calculations. Use get_element_global_stiffness_matrix
pub fn get_element_stiffness_matrix(element: &CalculationElement, 
    settings: &CalculationSettings
) -> DMatrix<f64> {
    let E = match &element.material {
        MaterialData::Concrete(c) => c.elastic_modulus,
        MaterialData::Steel(s) => s.elastic_modulus,
        MaterialData::Timber(_) => {
            println!("Timber is not yet implemented!");
            0.0
        }
    };
    let L = element.length;
    let A = element.profile.get_area(&element.material, settings);
    let I = element.profile.get_major_second_mom_of_area(&element.material, settings);
    let EA = E * A;
    let EI = E * I;
    let mut stiff_matrix = DMatrix::from_row_slice(
        6,
        6,
        &[
            EA / L,
            0.,
            0.,
            -EA / L,
            0.,
            0.,
            0.0,
            12.0 * EI / L.powi(3),
            6.0 * EI / L.powi(2),
            0.0,
            -12.0 * EI / L.powi(3),
            6.0 * EI / L.powi(2),
            0.0,
            6.0 * EI / L.powi(2),
            4.0 * EI / L,
            0.0,
            -6.0 * EI / L.powi(2),
            2.0 * EI / L,
            -EA / L,
            0.0,
            0.0,
            EA / L,
            0.0,
            0.0,
            0.0,
            -12.0 * EI / L.powi(3),
            -6.0 * EI / L.powi(2),
            0.0,
            12.0 * EI / L.powi(3),
            -6.0 * EI / L.powi(2),
            0.0,
            6.0 * EI / L.powi(2),
            2.0 * EI / L,
            0.0,
            -6.0 * EI / L.powi(2),
            4.0 * EI / L,
        ],
    );
    stiff_matrix = handle_releases(element, &stiff_matrix);
    return stiff_matrix;
}


/// Handles the releases in the stiffness matrix.
///
/// K = K<sub>pp</sub> - K<sub>pf</sub>\*K<sub>ff</sub><sup>-1</sup>*K<sub>fp</sub>
///
/// Where:
/// - K<sub>pp</sub> is the preserved stiffness matrix (the rows and columns that do not intersect with any of the
/// released rows and columns.
/// - K<sub>pf</sub> is the columns of the released directions but do not include the intersections of released
/// rows and columns.
/// - K<sub>fp</sub> is likewise but the rows
/// - K<sub>ff</sub> contains only the released cells, the intersections of the released rows and columns
///
/// For example, with end rotation released
/// - K<sub>ff</sub> would be K\[5,5] (zero start index)
/// - K<sub>pf</sub> would be K\[0..4, 5] (column 6 and rows 1 to 5)
/// - K<sub>pf</sub> would be K\[5, 0..4] (row 6 and columns 1 to 5)
/// - K<sub>pp</sub> would be K\[0..4, 0..4] (all cells but rows and columns 6)
/// - K<sub>ff</sub> would be the cells of K\[5,5] (row 6, column 6)
fn handle_releases(elem: &CalculationElement, stiff_matrix: &DMatrix<f64>) -> DMatrix<f64> {
    let dof = 3;
    let release_count = elem.releases.start_release_count() + elem.releases.end_release_count();
    // Kpp
    let mut preserved: DMatrix<f64> = DMatrix::zeros(dof*2, dof*2);
    // Kff (only the released cells, the intersections of the released rows and columns)
    let mut released: DMatrix<f64> = DMatrix::zeros(release_count, release_count);
    // Kpf and Kfp (released rows and columns but intersects with preserved rows and columns)
    let mut modifiers_cols: DMatrix<f64> = DMatrix::zeros(dof*2-release_count, release_count);
    let mut modifiers_rows: DMatrix<f64> = DMatrix::zeros(release_count, dof*2-release_count);

    let mut kff_row_cur = 0;
    let mut kff_col_cur = 0;
    let mut increm_kff_row_count = false;
    for i in 0..dof*2 {
        for j in 0..dof*2 {
            let rel_row = elem.releases.get_release_value(i).unwrap();
            let rel_col = elem.releases.get_release_value(j).unwrap();
            if rel_row && rel_col {
                // Move the intersected release values into Kff
                released[(kff_row_cur, kff_col_cur)] = stiff_matrix[(i, j)];
                kff_col_cur += 1;
                increm_kff_row_count = true;
            } else if rel_row {

            }
        }
        if increm_kff_row_count {
            kff_row_cur += 1;
            kff_col_cur = 0;
            increm_kff_row_count = false;
        }
    }

    println!("Kff: {:?}", released);

    // TODO Fix the return
    return stiff_matrix.clone();
}

pub(super) fn create_joined_stiffness_matrix(
    calc_model: &CalcModel,
    settings: &CalculationSettings
) -> (DMatrix<f64>, BTreeMap<i32, ReleaseIndexMap>) {
    let supp_count = calc_model.structure_nodes.len() + calc_model.extra_nodes.len();    
    // Increase the joined stiffness matrix size by release count. Releases are set into their
    // own rows and columns at the end of the joined matrix
    let release_count = crate::structure::utils::get_element_release_count(&calc_model.structure_elements);
    // The degrees of freedom count of single node (tx, tz, ry)
    let dof = 3;
    let row_width = supp_count * dof + release_count;

    let mut matrix_vector = vec![0.0; row_width * row_width];

    // The starting row and column locations for locating the cells for releases
    let mut rel_row = 0;
    let mut supp_index1: usize;
    let mut supp_index2: usize;
    let mut i_normalized: usize;
    let mut j_normalized: usize;

    // A map to store the index of the release column for each row. The key is the element number
    // and the ReleaseIndexMap contains the indexes for different releases
    let mut release_index_map: BTreeMap<i32, ReleaseIndexMap > = BTreeMap::new();
    // The number of releases in the global stiffness matrix
    let mut g_rel_increment_count = 0;

    for elem in calc_model.get_all_calc_elements() {
        release_index_map.insert(elem.model_el_num, ReleaseIndexMap::default());
        let e_glob_stiff_matrix = get_element_global_stiffness_matrix(&elem, settings);
        // The index of the start node
        let s = (elem.node_start - 1) as usize;
        // The index of the end node
        let e = (elem.node_end - 1) as usize;
        // The local release counter for the element
        let mut l_rel_increment_count = 0;
        for i in 0..dof * 2 {
            // Reset the column counter at every row change
            let mut rel_col = 0 + g_rel_increment_count;
            let mut increment_rel_row_count = false;
            for j in 0..dof * 2 {
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
                        j_normalized = j - dof;
                    }
                } else {
                    supp_index1 = e;
                    i_normalized = i - dof;
                    if j < dof {
                        // the top right triple (end element, start node)
                        supp_index2 = s;
                        j_normalized = j;
                    } else {
                        // the top right triple (end element, end node)
                        supp_index2 = e;
                        j_normalized = j - dof;
                    }
                }
                // If there is a release at either i or j, it needs to be handled
                if elem.releases.get_release_value(i).unwrap()
                    || elem.releases.get_release_value(j).unwrap()
                {
                    if i == j {
                        // If current row and column have release, place the value in the intersection of the current
                        // release row and column
                        matrix_vector[
                            row_width * supp_count * dof +    // Go to the start of the release columns
                                supp_count * dof + rel_col    // Move by the release column count
                            + row_width*rel_row               // Move by the release row count
                            ] += e_glob_stiff_matrix[(i, j)];
                        release_index_map.get_mut(&elem.model_el_num).unwrap().set(i, supp_count * dof + rel_col);
                        rel_col += 1;
                        l_rel_increment_count += 1;
                    } else if elem.releases.get_release_value(i).unwrap() {
                        // If the current row has a release, move the whole row to the rel_row
                        matrix_vector[
                            supp_count * dof +                // Go to the start of the release row
                            (supp_index2 * dof) * row_width + // Move by the node number
                            j_normalized * row_width +        // Move by the column number
                            rel_row]
                            +=
                            e_glob_stiff_matrix[(i, j)];
                        increment_rel_row_count = true;
                    } else if elem.releases.get_release_value(j).unwrap() {
                        // If the current column has a release, move the whole column to the rel_col
                        matrix_vector[
                            row_width * supp_count * dof +         // Move to start of release columns
                            (supp_index1 * dof) +                  // Move by the node number
                            rel_col * row_width                    // Move if there are multiple releases
                            + i_normalized]                        // Move by current row
                            += e_glob_stiff_matrix[(i, j)];

                        rel_col += 1;
                    }
                } else {
                    // (supp_index1 * dof) * row_width       offset the rows by the support node number
                    // supp_index2 * dof                     offset the columns by the support number
                    // j_normalized                          offset the columns by j
                    // i_normalized * row_width              offset the rows by i
                    matrix_vector[(supp_index1 * dof) * row_width
                        + i_normalized * row_width
                        + supp_index2 * dof
                        + j_normalized] += e_glob_stiff_matrix[(i, j)];
                }
            }
            // Before moving to new row, increase the current row count by the number of releases
            if increment_rel_row_count {
                rel_row += 1;
            }
        }
        g_rel_increment_count += l_rel_increment_count;
    }

    (DMatrix::from_vec(row_width, row_width, matrix_vector), release_index_map)
    // DMatrix::from_row_slice(row_width, row_width, &matrix_vector)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::fem::stiffness::{get_element_stiffness_matrix, handle_releases};
    use crate::material::{MaterialData, Steel};
    use crate::profile::Profile;
    use crate::settings::CalculationSettings;
    use crate::structure::{CalculationElement, Element};

    #[test]
    fn test_release_handling() {
        let elem = Element::new(1, 1, 2, Profile::new_rectangle("ASD".to_string(), 100.0, 100.0), MaterialData::Steel(Steel::new_s355()));
        let mut calc_elem = CalculationElement {
            calc_el_num: 1,
            model_el_num: 1,
            model_el_length: 4000.0,
            node_start: 1,
            node_end: 2,
            material: &Default::default(),
            profile: &Profile::new_rectangle("ASAD".to_string(), 100.0, 100.0),
            releases: Default::default(),
            length: 0.0,
            rotation: 0.0,
            profile_area: 0.0,
            elastic_modulus: 0.0,
            major_smoa: 0.0,
            offset_from_model_el: 0.0,
        };
        calc_elem.releases.e_ry = true;
        handle_releases(&calc_elem, &get_element_stiffness_matrix(&calc_elem, &CalculationSettings::default()));
    }
}