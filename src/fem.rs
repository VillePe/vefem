#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;

use vputilslib::equation_handler::EquationHandler;

use crate::fem::fem_handler::*;
use crate::fem::stiffness::*;
use crate::loads;
use crate::{
    loads::{load::CalculationLoad, Load},
    results::*,
    settings::{self, CalculationSettings},
    structure::{Element, Node},
};

pub mod axial_deformation;
pub mod deflection;
pub mod equivalent_loads;
pub mod fem_handler;
pub mod internal_forces;
pub mod matrices;
pub mod stiffness;
pub mod utils;

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
    calc_settings: &CalculationSettings,
) -> CalculationResults {
    let col_height = utils::col_height(nodes, elements);

    let calculation_loads =
        &loads::utils::extract_calculation_loads(elements, nodes, loads, equation_handler);

    let mut global_stiff_matrix = create_joined_stiffness_matrix(elements, nodes);
    // The global equivalent loads matrix
    let global_eq_l_matrix = equivalent_loads::create(elements, nodes, calculation_loads);
    let displacements = calculate_displacements(
        nodes,
        col_height,
        &mut global_stiff_matrix,
        &global_eq_l_matrix,
    );
    let reactions = calculate_reactions(
        &global_stiff_matrix,
        &displacements,
        &global_eq_l_matrix,
    );

    let node_results = NodeResults::new(displacements, reactions, nodes.len(), &equation_handler);
    let internal_force_results = calc_internal_forces(
        elements,
        nodes,
        calculation_loads,
        &node_results,
        calc_settings,
    );

    CalculationResults {
        node_results,
        internal_force_results,
    }
}

fn calc_internal_forces(
    elements: &Vec<Element>,
    nodes: &HashMap<i32, Node>,
    loads: &Vec<CalculationLoad>,
    node_results: &NodeResults,
    calc_settings: &CalculationSettings,
) -> HashMap<i32, InternalForceResults> {
    let mut map: HashMap<i32, InternalForceResults> = HashMap::new();
    for element in elements {
        let element_length = element.get_length(nodes);
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
        while x <= element_length && !last {
            let moment_force_val =
                internal_forces::calculate_moment_at(x, element, nodes, loads, node_results);
            let axial_force_val =
                internal_forces::calculate_axial_force_at(x, element, nodes, loads, node_results);
            let shear_force_val =
                internal_forces::calculate_shear_at(x, element, nodes, loads, node_results);
            let deflection_val = deflection::calculate_at(x, element, nodes, loads, node_results);
            let axial_deformation_val =
                axial_deformation::calculate_at(x, element, nodes, loads, node_results);

            moment_forces.push(InternalForcePoint {
                force_type: ForceType::Moment,
                value_x: 0.0,
                value_y: moment_force_val,
                pos_on_element: x,
                element_number: element.number,
                load_comb_number: 0,
            });
            axial_forces.push(InternalForcePoint {
                force_type: ForceType::Axial,
                value_x: 0.0,
                value_y: axial_force_val,
                pos_on_element: x,
                element_number: element.number,
                load_comb_number: 0,
            });
            shear_forces.push(InternalForcePoint {
                force_type: ForceType::Shear,
                value_x: 0.0,
                value_y: shear_force_val,
                pos_on_element: x,
                element_number: element.number,
                load_comb_number: 0,
            });
            deflections.push(InternalForcePoint {
                force_type: ForceType::Deflection,
                value_x: axial_deformation_val,
                value_y: deflection_val,
                pos_on_element: x,
                element_number: element.number,
                load_comb_number: 0,
            });

            x += split_interval;
            if x >= element_length {
                x = element_length;
                last = true;
            }
        }

        let res = InternalForceResults {
            element_number: element.number,
            axial_forces,
            shear_forces,
            moment_forces,
            deflections,
        };
        map.insert(element.number, res);
    }

    map
}
