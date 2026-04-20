use std::collections::BTreeMap;

pub fn col_height(nodes: &BTreeMap<i32, crate::structure::Node>) -> usize {
    // The degrees of freedom count of single node (tx, tz, ry)
    let dof = 3;
    let col_height = nodes.len() * dof;
    col_height
}