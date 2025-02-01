#![allow(dead_code)]

use std::collections::HashMap;
use nalgebra::DMatrix;
use crate::structure::element::{Element};
use crate::structure::Node;

/// Gets the rotation matrix for the element. This matrix is in elements local coordinate system
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