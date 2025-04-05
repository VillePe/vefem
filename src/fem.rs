#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::collections::HashMap;

pub use calculation_model::CalcModel;
use vputilslib::equation_handler::EquationHandler;

use crate::fem::fem_handler::*;
use crate::fem::stiffness::*;
use crate::loads;
use crate::loads::LoadCombination;
use crate::structure::StructureModel;
use crate::{
    loads::load::CalculationLoad,
    results::*,
    settings::{self, CalculationSettings},
};

pub mod axial_deformation;
pub mod deflection;
pub mod equivalent_loads;
pub mod fem_handler;
pub mod internal_forces;
pub mod matrices;
pub mod stiffness;
pub mod utils;
mod calculation_model;

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
    let (calc_elements, extra_nodes) = crate::structure::utils::get_calc_elements(elements, nodes, &HashMap::new(), calc_settings);
    let calc_model = CalcModel::new(&nodes, extra_nodes, &elements, calc_elements);

    let col_height = utils::col_height(nodes, elements);
    
    let load_combinations = if struct_model.load_combinations.is_empty() {
        &vec![LoadCombination::default()]
    } else {
        &struct_model.load_combinations
    };
    
    let mut results: Vec<CalculationResults> = Vec::new();
    for lc in load_combinations {
        let calculation_loads =
        &loads::utils::extract_calculation_loads(&calc_model, loads, lc, equation_handler);

        let mut global_stiff_matrix = create_joined_stiffness_matrix(&calc_model, calc_settings);
        // The global equivalent loads matrix
        let global_eq_l_matrix = equivalent_loads::create(&calc_model, calculation_loads, calc_settings);
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
        let internal_force_results = calc_internal_forces(
            &calc_model,
            calculation_loads,
            &node_results,
            calc_settings,
        );

        let result = CalculationResults {
            load_combination: lc.name.clone(),
            node_results,
            internal_force_results,
        };
        results.push(result);
    }
    results
}

/// Calculates the internal forces for the elements by support reactions and displacements in node results
/// 
/// Parameters:
/// * 'elements' - the elements of the structure model
/// * 'nodes' - the nodes of the structure model
/// * 'loads' - the loads of the calculation model
/// * 'node_results' - the node results of the structure
/// * 'calc_settings' - the calculation settings
/// Returns: BTreeMap<i32, InternalForceResults> where the key is the element number
fn calc_internal_forces(
    calc_model: &CalcModel,
    loads: &Vec<CalculationLoad>,
    node_results: &NodeResults,
    calc_settings: &CalculationSettings,
) -> BTreeMap<i32, InternalForceResults> {
    let mut map: BTreeMap<i32, InternalForceResults> = BTreeMap::new();
    for element in calc_model.calc_elements.iter() {
        let element_length = element.length;
        let split_interval = match calc_settings.calc_split_interval {
            settings::calc_settings::CalcSplitInterval::Absolute(a) => a,
            settings::calc_settings::CalcSplitInterval::Relative(r) => element_length * r,
        };
        let mut moment_forces = vec![];
        let mut shear_forces = vec![];
        let mut axial_forces = vec![];
        let mut deflections = vec![];
        let mut x = 0.0;
        let mut last = false;
        while x < element_length || last {
            let moment_force_val =
                internal_forces::calculate_moment_at(x, element, loads, node_results);
            let axial_force_val =
                internal_forces::calculate_axial_force_at(x, element, loads, node_results);
            let shear_force_val =
                internal_forces::calculate_shear_at(x, element, loads, node_results);
            let deflection_val = deflection::calculate_at(x, element, loads, calc_settings, node_results);
            let axial_deformation_val =
                axial_deformation::calculate_at(x, element, loads, calc_settings, node_results);

            moment_forces.push(InternalForcePoint {
                force_type: ForceType::Moment,
                value_x: 0.0,
                value_y: moment_force_val,
                pos_on_element: x, // TODO ADD THE OFFSET OF CALC ELEMENT TO MODEL ELEMENT START
                element_number: element.model_el_num,
                load_comb_number: 0,
            });
            axial_forces.push(InternalForcePoint {
                force_type: ForceType::Axial,
                value_x: 0.0,
                value_y: axial_force_val,
                pos_on_element: x,
                element_number: element.model_el_num,
                load_comb_number: 0,
            });
            shear_forces.push(InternalForcePoint {
                force_type: ForceType::Shear,
                value_x: 0.0,
                value_y: shear_force_val,
                pos_on_element: x,
                element_number: element.model_el_num,
                load_comb_number: 0,
            });
            deflections.push(InternalForcePoint {
                force_type: ForceType::Deflection,
                value_x: axial_deformation_val,
                value_y: deflection_val,
                pos_on_element: x,
                element_number: element.model_el_num,
                load_comb_number: 0,
            });

            x += split_interval;

            // Make sure that last point is exactly at the end of the element
            if last {
                break;
            }
            if x >= element_length {
                x = element_length;
                last = true;
            }
        }

        let res = InternalForceResults {
            element_number: element.model_el_num,
            axial_forces,
            shear_forces,
            moment_forces,
            deflections,
        };
        map.insert(element.model_el_num, res);
    }

    map
}
