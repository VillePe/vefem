use std::collections::BTreeMap;

use crate::structure::Element;

pub fn col_height(nodes: &BTreeMap<i32, crate::structure::Node>, elements: &Vec<Element>) -> usize {
    // Increase the joined stiffness matrix size by release count. Releases are set into their
    // own rows and columns at the end of the joined matrix
    let release_count = crate::structure::utils::get_element_release_count(elements);
    // The degrees of freedom count of single node (tx, tz, ry)
    let dof = 3;
    let col_height = nodes.len() * dof + release_count;
    col_height
}