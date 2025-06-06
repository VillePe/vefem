﻿#![allow(dead_code)]

use nalgebra::DMatrix;
use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use vputilslib::equation_handler::EquationHandler;

use super::CalcModel;
use crate::loads::{CalcLoadCombination, Load};
use crate::settings::CalculationSettings;
use crate::{
    fem::{
        equivalent_loads,
        internal_forces::calc_internal_forces,
        matrices::{
            get_unknown_translation_eq_loads_rows, get_unknown_translation_rows,
            get_unknown_translation_stiffness_rows,
        },
        stiffness::create_joined_stiffness_matrix,
    },
    loads,
    loads::LoadCombination,
    results::{CalculationResults, NodeResults},
    structure::{Node, StructureModel},
};

/// Calculates the displacements, support reactions and element internal forces.
/// * 'calc_model' - calculation model that is extracted to calculation objects
/// * 'equation_handler' - equation handler that can contain custom variables set by the user.
/// The 'L' variable is reserved for the length of the element.
pub fn calculate(
    struct_model: &StructureModel,
    equation_handler: &EquationHandler,
) -> Vec<CalculationResults> {
    let nodes = &struct_model.nodes;
    let elements = &struct_model.elements;
    let loads = &struct_model.loads;
    let calc_settings = &struct_model.calc_settings;
    let (calc_elements, extra_nodes) =
        crate::structure::utils::get_calc_elements(elements, nodes, &HashMap::new(), calc_settings);
    let calc_model = CalcModel::new(&nodes, extra_nodes, &elements, calc_elements);

    let col_height = super::utils::col_height(nodes, elements);

    let load_combinations = if struct_model.load_combinations.is_empty() {
        &vec![LoadCombination::default()]
    } else {
        &struct_model.load_combinations
    };

    let results: Arc<Mutex<Vec<CalculationResults>>> = Arc::new(Mutex::new(Vec::new()));

    let calc_model = &calc_model;
    let equation_handler = &equation_handler;
    let result_clone = results.clone();
    thread::scope(move |s| {
        for model_lc in load_combinations {
            let calc_load_combinations = loads::lc_utils::get_calc_load_combinations(
                model_lc,
                &loads
            );
            for lc in calc_load_combinations.into_iter() {
                let result_clone = result_clone.clone();
                if calc_settings.calc_threaded {
                    s.spawn(move || {
                        calc_lc(calc_model, loads, lc, equation_handler, result_clone, calc_settings, nodes, col_height);
                    });
                } else {
                    calc_lc(calc_model, loads, lc, equation_handler, result_clone, calc_settings, nodes, col_height);
                }
            }
        }
    });
    let mut result_list = Arc::try_unwrap(results)
        .unwrap()
        .into_inner()
        .expect("REASON");

    // Sort the results by sub load combination number
    result_list.sort_by(|a, b| a.sub_load_comb_num.cmp(&b.sub_load_comb_num));

    result_list
}

fn calc_lc(
    calc_model: &CalcModel,
    loads: &Vec<Load>,
    lc: CalcLoadCombination,
    equation_handler: &EquationHandler,
    result_clone: Arc<Mutex<Vec<CalculationResults>>>,
    calc_settings: &CalculationSettings,
    nodes: &BTreeMap<i32, Node>,
    col_height: usize,
) {
    let calculation_loads =
        &loads::utils::extract_calculation_loads(calc_model, loads, &lc, equation_handler);

    let mut global_stiff_matrix = create_joined_stiffness_matrix(calc_model, calc_settings);
    // The global equivalent loads matrix
    let global_eq_l_matrix =
        equivalent_loads::create(calc_model, calculation_loads, calc_settings);
    let displacements = calculate_displacements(
        nodes,
        col_height,
        &mut global_stiff_matrix,
        &global_eq_l_matrix,
    );

    let reactions = calculate_reactions(&global_stiff_matrix, &displacements, &global_eq_l_matrix);

    let displacements = displacements.column(0).as_slice().to_vec();
    let reactions = reactions.column(0).as_slice().to_vec();

    let node_results = NodeResults::new(displacements, reactions, nodes.len(), &equation_handler);
    let internal_force_results =
        calc_internal_forces(calc_model, calculation_loads, &node_results, calc_settings);

    let result = CalculationResults {
        load_combination: lc.parent_load_combination.clone(),
        load_comb_num: lc.parent_load_combination_number,
        sub_load_comb_num: lc.sub_number,
        node_results,
        internal_force_results,
    };
    result_clone.deref().lock().unwrap().push(result);
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
    nodes: &BTreeMap<i32, Node>,
    col_height: usize,
    global_stiff_matrix: &mut DMatrix<f64>,
    global_equivalent_loads_matrix: &DMatrix<f64>,
) -> DMatrix<f64> {
    apply_support_spring_values(nodes, global_stiff_matrix);
    // Get the rows with unknown translations to calculate the displacements for them.
    let unknown_translation_rows = get_unknown_translation_rows(nodes, &global_stiff_matrix);
    let unknown_translation_stiffness_rows =
        get_unknown_translation_stiffness_rows(&unknown_translation_rows, &global_stiff_matrix);
    let unknown_eq_loads_rows = get_unknown_translation_eq_loads_rows(
        &unknown_translation_rows,
        &global_equivalent_loads_matrix,
    );
    let displacement: DMatrix<f64>;
    // If there are big number of rows with unknown translations, use cholesky decomposition for
    // solving the system of equations. Otherwise use regular inversion (might not be necessary,
    // maybe could always solve with cholesky. Could be benchmarked).
    if unknown_translation_stiffness_rows.nrows() > 100 {
        displacement =
            displacements_cholesky(unknown_translation_stiffness_rows, &unknown_eq_loads_rows)
                .unwrap_or(DMatrix::zeros(col_height, 1));
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
    remove_support_spring_values(nodes, global_stiff_matrix);

    full_displacement_matrix
}

fn apply_support_spring_values(
    nodes: &BTreeMap<i32, Node>,
    global_stiff_matrix: &mut DMatrix<f64>,
) {
    let dof = 3;
    for node in nodes.values() {
        for i in 0..dof {
            if node.support.get_support_spring(i) != 0.0 && node.number > 0 {
                let node_number = node.number as usize;
                global_stiff_matrix[((node_number - 1) * dof + i, (node_number - 1) * dof + i)] +=
                    node.support.get_support_spring(i);
            }
        }
    }
}

fn remove_support_spring_values(
    nodes: &BTreeMap<i32, Node>,
    global_stiff_matrix: &mut DMatrix<f64>,
) {
    let dof = 3;
    for node in nodes.values() {
        for i in 0..dof {
            if node.support.get_support_spring(i) != 0.0 && node.number > 0 {
                let node_number = node.number as usize;
                global_stiff_matrix[((node_number - 1) * dof + i, (node_number - 1) * dof + i)] -=
                    node.support.get_support_spring(i);
            }
        }
    }
}

/// Calculates the displacements using cholesky decomposition for given rows that have unknown translations
/// * 'unknown_translation_stiffness_rows' - stiffness matrix rows that have unknown translations
/// * 'unknown_eq_loads' - equivalent loads at the same rows as the stiffness matrix
fn displacements_cholesky(
    unknown_translation_stiffness_rows: DMatrix<f64>,
    unknown_eq_loads: &DMatrix<f64>,
) -> Option<DMatrix<f64>> {
    match unknown_translation_stiffness_rows.cholesky() {
        Some(cholesky) => Some(cholesky.solve(&unknown_eq_loads)),
        None => None,
    }
}

/// Creates the inverted stiffness matrix for given matrix. If the matrix is larger than 100x100,
/// cholesky decomposition is used for inversion. Otherwise regular inversion is used.
/// * 'matrix' - matrix to invert (should be the stiffness matrix with uknonwn translations)
fn invert_stiff_matrix(matrix: DMatrix<f64>) -> Option<DMatrix<f64>> {
    // println!("Using regular inversion...");
    return matrix.try_inverse();
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
    use std::{collections::BTreeMap, time::SystemTime};

    use vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};

    use crate::{
        loads::{Load, LoadGroup},
        material::{MaterialData, Steel},
        profile::Profile,
        settings::CalculationSettings,
        structure::{Element, Node, StructureModel},
    };

    // #[test]
    fn t_simple_benchmark_calculation() {
        let mut elements: Vec<Element> = vec![];
        let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        // Create multiple 4 meter long elements to test the speed of calculations
        for i in 0..1000 {
            nodes.insert(
                i + 2,
                Node::new_hinged(i + 2, VpPoint::new(((i + 1) as f64) * 4000.0f64, 0.0)),
            );
            elements.push(Element::new(
                i + 1,
                i + 1,
                i + 2,
                Profile::new_rectangle("100x100".to_string(), 100.0, 100.0),
                MaterialData::Steel(Steel::new(210e3)),
            ));
        }
        let timer = SystemTime::now();
        let load = Load::new_line_load(
            "Lineload".to_string(),
            "-1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
            LoadGroup::PERMANENT,
        );

        let calc_model = StructureModel {
            nodes,
            elements,
            loads: vec![load],
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let results = crate::fem::fem_handler::calculate(&calc_model, &mut EquationHandler::new());
        println!("Calculation time: {:?}", timer.elapsed().unwrap());
        println!("Element count: {}", calc_model.elements.len());
        println!("Node count: {}", calc_model.nodes.len());
        println!(
            "Result displacement row count: {:?}",
            results[0].node_results.displacements.len()
        );
        println!(
            "Support reaction (0,1): {} kN",
            results[0].node_results.get_support_reaction(1, 1) / 1000.0
        );
    }
}
