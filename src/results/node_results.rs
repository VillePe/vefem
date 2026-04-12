use std::collections::BTreeMap;
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use vputilslib::equation_handler::EquationHandler;

use crate::{fem::stiffness, loads::load::CalculationLoad, structure::CalculationElement};
use crate::structure::element::ReleaseIndexMap;
use crate::structure::Node;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResults {
    /// The displacements at the nodes. Note that the displacements are in the local coordinate system of the nodes.
    /// If the node is rotated these values are rotated accordingly
    pub displacements: Vec<f64>,
    /// The support reactions at the nodes. Note that the displacements are in the local coordinate system of the nodes.
    /// If the node is rotated these values are rotated accordingly
    pub support_reactions: Vec<f64>,
    /// These displacements are guaranteed to be in the global coordinate system
    pub global_displacements: Vec<f64>,
    pub node_count: usize,
    pub dof_count: usize,
    pub equation_handler: EquationHandler,
    pub release_index_map: BTreeMap<i32, ReleaseIndexMap>,
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
        nodes: &BTreeMap<i32, Node>,
        release_index_map: BTreeMap<i32, ReleaseIndexMap>,
    ) -> Self {
        let mut copied_eq_handler = EquationHandler::new();
        let dof: usize = 3;

        let variables = equation_handler.get_variables();
        for key in variables.keys() {
            copied_eq_handler.set_variable(key, variables[key]);
        }
        let mut global_displacements = Vec::with_capacity(displacements.len());
        for _ in 0..displacements.len() {
            global_displacements.push(0.0);
        }
        // Add the rotated values to global displacement matrix
        for node in nodes.values() {
            let node_number: usize = node.number as usize;
            if node.support.rotation != 0.0 {
                let cos = node.support.rotation.to_radians().cos();
                let sin = node.support.rotation.to_radians().sin();
                // Transpose of rotation matrix * small_displacement
                // x = r11 * sm1 + r12 * sm2 (r11 = cos, r12 = -sin)
                // y = r21 * sm1 + r22 * sm2 (r21 = sin, r22 = cos)
                let displacement_x = cos * displacements[(node_number-1)*dof] + -sin * displacements[(node_number-1)*dof+1];
                let displacement_y = sin * displacements[(node_number-1)*dof] + cos * displacements[(node_number-1)*dof+1];
                // Rotation displacement is not affected by rotation of support
                global_displacements[(node_number-1)*dof+0] = displacement_x;
                global_displacements[(node_number-1)*dof+1] = displacement_y;
                global_displacements[(node_number-1)*dof+2] = displacements[(node_number-1)*dof+2];
            } else {
                global_displacements[(node_number-1)*dof] = displacements[(node_number-1)*dof];
                global_displacements[(node_number-1)*dof+1] = displacements[(node_number-1)*dof+1];
                global_displacements[(node_number-1)*dof+2] = displacements[(node_number-1)*dof+2];
            }
        }
        // Add possible release values to global displacement matrix
        for i in nodes.len()*dof..displacements.len() {
            // TODO should these be rotated to be truly global? Now they will just be copied and might be in element local coordinates. But does that matter?
            // Would probably need a reverse map for release index map to get the element rotation
            // and apply that to this displacement value.
            global_displacements[i] = displacements[i];
        }


        Self {
            displacements,
            support_reactions,
            node_count,
            equation_handler: copied_eq_handler,
            dof_count: 3,
            global_displacements,
            release_index_map,
        }
    }

    /// Get the displacements at given node number and direction. If the support is rotated,
    /// the returned displacement will also be
    /// The direction is as follows:
    /// - 0 = translation in X-axis,
    /// - 1 = translation in Z-axis,
    /// - 2 = rotation about Y-axis.
    pub fn get_local_displacement(&self, node_number: i32, dir: usize) -> f64 {
        self.displacements[((node_number - 1) * self.dof_count as i32 + dir as i32) as usize]
    }

    /// Get the displacements at given node number and direction. Even if the support is rotated,
    /// the returned displacements will be in global coordinates
    /// The direction is as follows:
    /// - 0 = translation in X-axis,
    /// - 1 = translation in Z-axis,
    /// - 2 = rotation about Y-axis.
    pub fn get_global_displacement(&self, node_number: i32, dir: usize) -> f64 {
        self.global_displacements[((node_number - 1) * self.dof_count as i32 + dir as i32) as usize]
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
        release_index_map: &ReleaseIndexMap,
        settings: &crate::settings::CalculationSettings
    ) -> DMatrix<f64> {
        let el_stiff_matrix = stiffness::get_element_stiffness_matrix(element, settings);
        let el_eq_loads = crate::fem::equivalent_loads::get_element_g_eq_loads(element, loads, settings);
        let rot_matrix = crate::fem::matrices::get_rotation_matrix(element.rotation);
        let local_displacements = self.get_elem_local_displacements(element, release_index_map);

        el_stiff_matrix * local_displacements - rot_matrix * el_eq_loads
    }

    /// Get the local displacement matrix for the element
    pub fn get_elem_local_displacements(
        &self,
        element: &CalculationElement,
        release_index_map: &ReleaseIndexMap
    ) -> DMatrix<f64> {
        let mut global_matrix = DMatrix::<f64>::zeros(6, 1);

        // TODO Shouldn't this take the releases into account?
        for i in 0..self.dof_count {
            if element.releases.get_release_value(i).unwrap() {
                // If the start node is released, the displacement is in a different index than
                // the node number
                global_matrix[(i, 0)] = self.global_displacements[(release_index_map.get(i)) as usize]
            } else {
                global_matrix[(i, 0)] = self.get_global_displacement(element.node_start, i);
            }
        }
        for i in 0..self.dof_count {
            if element.releases.get_release_value(i+self.dof_count).unwrap() {
                // If the end node is released, the displacement is in a different index than
                // the node number
                global_matrix[(self.dof_count + i, 0)] = self.global_displacements[
                    release_index_map.get(i+self.dof_count) as usize
                ]
            } else {
                global_matrix[(self.dof_count + i, 0)] = self.get_global_displacement(element.node_end, i);
            }
        }
        let rot_matrix = crate::fem::matrices::get_rotation_matrix(element.rotation);

        rot_matrix * global_matrix
    }
}