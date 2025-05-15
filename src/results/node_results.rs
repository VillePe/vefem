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
        let rot_matrix = crate::fem::matrices::get_element_rotation_matrix(element);
        let local_displacements = self.get_elem_local_displacements(element);        

        el_stiff_matrix * local_displacements - rot_matrix * el_eq_loads
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