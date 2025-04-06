use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use vputilslib::equation_handler::EquationHandler;

use crate::{fem::stiffness, loads::load::CalculationLoad, structure::CalculationElement};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResults {
    pub displacements: Vec<f64>,
    pub support_reactions: Vec<f64>,
    pub node_count: usize,
    pub dof_count: usize,
    pub equation_handler: EquationHandler,
}

impl NodeResults {
    /// Initializes a new instance of NodeResults
    /// * 'displacements' - the displacements matrix
    /// * 'support_reactions' - the support reactions matrix
    /// * 'node_count' - the number of nodes
    /// * 'equation_handler' - the equation handler. This equation handler is copied to the new instance.
    pub fn new(
        displacements: Vec<f64>,
        support_reactions: Vec<f64>,
        node_count: usize,
        equation_handler: &EquationHandler,
    ) -> Self {
        let mut copied_eq_handler = EquationHandler::new();

        let variables = equation_handler.get_variables();
        for key in variables.keys() {
            copied_eq_handler.set_variable(key, variables[key]);
        }
        Self {
            displacements,
            support_reactions,
            node_count,
            equation_handler: copied_eq_handler,
            dof_count: 3,
        }
    }

    /// Get the displacements at given node number and direction.
    /// The direction is as follows:
    /// - 0 = translation in X-axis,
    /// - 1 = translation in Z-axis,
    /// - 2 = rotation about Y-axis.
    pub fn get_displacement(&self, node_number: i32, dir: usize) -> f64 {
        self.displacements[((node_number - 1) * self.dof_count as i32 + dir as i32) as usize]
    }

    /// Get the support reactions at given node number and direction.
    /// The direction is as follows:
    /// - 0 = reaction in X-axis,
    /// - 1 = reaction in Z-axis,
    /// - 2 = moment about Y-axis.
    pub fn get_support_reaction(&self, node_number: i32, dir: usize) -> f64 {
        self.support_reactions[((node_number - 1) * self.dof_count as i32 + dir as i32) as usize]
    }

    /// Get the local nodal force vectors for the element
    pub fn get_elem_local_nodal_force_vectors(
        &self,
        element: &CalculationElement,
        loads: &Vec<CalculationLoad>,
        settings: &crate::settings::CalculationSettings
    ) -> DMatrix<f64> {
        let el_stiff_matrix = stiffness::get_element_stiffness_matrix(element, settings);
        let el_eq_loads = crate::fem::equivalent_loads::get_element_g_eq_loads(element, loads, settings);
        let local_displacements = self.get_elem_local_displacements(element);

        el_stiff_matrix * local_displacements - el_eq_loads
    }

    /// Get the local displacement matrix for the element
    pub fn get_elem_local_displacements(
        &self,
        element: &CalculationElement,
    ) -> DMatrix<f64> {
        let mut global_matrix = DMatrix::<f64>::zeros(6, 1);

        for i in 0..self.dof_count {
            global_matrix[(i, 0)] = self.get_displacement(element.node_start, i);
        }
        for i in 0..self.dof_count {
            global_matrix[(self.dof_count + i, 0)] = self.get_displacement(element.node_end, i);
        }
        let rot_matrix = crate::fem::matrices::get_element_rotation_matrix(element);

        rot_matrix * global_matrix
    }
}

#[cfg(test)]
mod tests {
    
    use std::collections::{BTreeMap, HashMap};
    use vputilslib::equation_handler::EquationHandler;

    use vputilslib::geometry2d::VpPoint;

    use crate::fem::CalcModel;
    use crate::loads::{Load, LoadCombination};
    use crate::material::{MaterialData, Steel};
    use crate::profile::Profile;
    use crate::settings::CalculationSettings;
    use crate::structure::StructureModel;
    use crate::structure::{Element, Node};

    #[test]
    fn t_get_elem_local_reactions() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(0000.0, 4000.0))),
        ]);
        let pl = Load::new_point_load(
            "name".to_string(),
            "1".to_string(),
            "1000".to_string(),
            "10".to_string(),
            00.0,
        );
        let elements = vec![el];
        let struct_model = StructureModel {
            nodes,
            elements: elements,
            loads: vec![pl],
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let (calc_elements, extra_nodes) = crate::structure::utils::get_calc_elements(
            &struct_model.elements, 
            &struct_model.nodes, 
            &HashMap::new(), 
            &struct_model.calc_settings);
        let calc_model = CalcModel::new(
            &struct_model.nodes, 
            extra_nodes, 
            &struct_model.elements, 
            calc_elements
        );
        let results = crate::fem::fem_handler::calculate(&struct_model, &mut EquationHandler::new());
        let calc_loads = crate::loads::utils::extract_calculation_loads(&calc_model, 
            &struct_model.loads, 
            &LoadCombination::default(), 
            &EquationHandler::new()
        );
        let local_reactions = results[0]
            .node_results
            .get_elem_local_nodal_force_vectors(&calc_model.get_all_calc_elements()[0], &calc_loads, &struct_model.calc_settings);
        println!("Local reactions: {:.0}", local_reactions);
    }
}
