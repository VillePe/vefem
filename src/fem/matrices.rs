use std::collections::HashMap;
use nalgebra::DMatrix;
use crate::structure::element::{Element, Material};
use crate::structure::node::Node;

pub fn get_element_global_stiffness_matrix(element: &Element, nodes: &HashMap<i32, Node>) -> DMatrix<f64> {
    let E = match &element.material {
        Material::Concrete(c) => {c.elastic_modulus}
        Material::Steel(s) => {s.elastic_modulus}
        Material::Timber(_) => {0.0}
    };
    let L = element.get_length(nodes);
    let A = element.profile.get_area();
    let I = element.profile.get_major_second_mom_of_area();
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

pub fn get_element_rotation_matrix(element: &Element, nodes: &HashMap<i32, Node>) -> DMatrix<f64> {
    let angle_radians = element.get_rotation(nodes).to_radians();
    let c = angle_radians.cos();
    let s = angle_radians.sin();
    DMatrix::from_row_slice(6,6, &[
        c  , s  , 0.0, 0.0, 0.0, 0.0,
        -s , c  , 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, c  , s  , 0.0,
        0.0, 0.0, 0.0, -s , c  , 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
}