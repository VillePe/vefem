use std::collections::{BTreeMap, HashMap, HashSet};

use vputilslib::geometry2d;

use crate::structure::element::Element;

use super::{element::CalculationElement, Node};

pub fn get_element_release_count(elements: &Vec<Element>) -> usize {
    let mut count = 0;
    for e in elements {
        if e.releases.s_tx {
            count += 1;
        }
        if e.releases.s_tz {
            count += 1;
        }
        if e.releases.s_ry {
            count += 1;
        }
        if e.releases.e_tx {
            count += 1;
        }
        if e.releases.e_tz {
            count += 1;
        }
        if e.releases.e_ry {
            count += 1;
        }
    }
    count
}

/// Creates the calculation elements based on nodes and split positions. Elements that have a node
/// located somewhere 'in' the element, the element is split there to two calculation elements.
/// Elements are also split by the given split positions that is a map of element numbers and the
/// position where the element should be split. It can be used to split elements in to multiple
/// calcuation elements if needed or, for example, split the element where a crack is located
/// (to calculate the eurocode deflections of a concrete beam).
///
/// ## Parameters:
/// * 'elements' - the elements of the structure model
/// * 'nodes' - the nodes of the structure model
/// * 'split_positions' - a map of element numbers and the position where the element should be split (in millimeters)
///
/// Returns: Vec<CalculationElement>
pub fn get_calc_elements(
    elements: &Vec<Element>,
    nodes: &BTreeMap<i32, Node>,
    split_positions: &HashMap<i32, i64>,
) -> (Vec<CalculationElement>, BTreeMap<i32, Node>) {
    let mut calc_elements: Vec<CalculationElement> = Vec::new();
    let mut extra_nodes: BTreeMap<i32, Node> = BTreeMap::new();
    for e in elements {
        let mut e_split_set: HashSet<i64> = HashSet::new();
        if split_positions.contains_key(&e.number) {
            let split_pos = *split_positions.get(&e.number).unwrap();
            // Only insert the value if it is not already in the set
            if !e_split_set.contains(&split_pos) {
                e_split_set.insert(split_pos);
            }
        }
        let e_start = &nodes.get(&e.node_start).unwrap().point;
        let e_end = &nodes.get(&e.node_end).unwrap().point;
        for n in nodes.values() {
            if geometry2d::point_in_line(e_start, e_end, &n.point, 0.1) {
                // Get the split position by calculating the length between the node and the start of the element
                let split_pos = geometry2d::calc_length_between_points(&n.point, e_start)
                    .abs()
                    .round() as i64;
                // Only insert the value if it is not already in the set
                if !e_split_set.contains(&split_pos) {
                    e_split_set.insert(split_pos);
                }
            }
        }
        for split_pos in e_split_set {
            
        }
    }
    (calc_elements, extra_nodes)
}
