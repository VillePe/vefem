use std::collections::{BTreeMap, HashMap};

use vputilslib::geometry2d::{self, VpPoint};

use crate::{settings::CalculationSettings, structure::element::Element};

use super::{element::CalculationElement, Node, Release};

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
pub fn get_calc_elements<'a>(
    elements: &'a Vec<Element>,
    nodes: &BTreeMap<i32, Node>,
    split_positions: &HashMap<i32, i64>,
    calc_settings: &CalculationSettings,
) -> (Vec<CalculationElement<'a>>, BTreeMap<i32, Node>) {
    let mut calc_elements: Vec<CalculationElement> = Vec::new();
    let mut extra_nodes: BTreeMap<i32, Node> = BTreeMap::new();
    let mut el_num = 1001;
    for e in elements {
        let mut e_split_set: HashMap<i64, &Node> = HashMap::new();

        let e_start = &nodes.get(&e.node_start).unwrap().point;
        let e_end = &nodes.get(&e.node_end).unwrap().point;
        let rotation = geometry2d::get_angle_from_points(e_start, e_end);
        let length = e.get_length(nodes);
        for n in nodes.values() {
            if n.number == e.node_start || n.number == e.node_end {
                continue;
            }
            if geometry2d::point_in_line(e_start, e_end, &n.point, 0.1) {
                // Get the split position by calculating the length between the node and the start of the element
                let split_pos = geometry2d::calc_length_between_points(&n.point, e_start)
                    .abs()
                    .round() as i64;
                // Only insert the value if it is not already in the set
                if !e_split_set.contains_key(&split_pos) {
                    e_split_set.insert(split_pos, &n);
                }
            }
        }

        if split_positions.contains_key(&e.number) {
            let split_pos = *split_positions.get(&e.number).unwrap();
            // Only insert the value if it is not already in the set
            if !e_split_set.contains_key(&split_pos) {
                // Create a node point at start of element.X + split_pos
                let mut node_point = VpPoint::new(e_start.x + split_pos as f64 , 0.0);
                // Rotate the point around the start point to move it to be in the element
                node_point = geometry2d::rotate_point(&e_start, &node_point, rotation);
                // Get the number for the support from node and extra node count
                let number = (nodes.len() + 1 + extra_nodes.len()) as i32;
                // Create new node and insert it into the extra nodes
                let n = Node::new_free(number, node_point);
                extra_nodes.insert(n.number, n);
                // Add the split pos into map and add a reference to the node inside the extra nodes map
                // This makes sure that the node lives longer than this if statement
                e_split_set.insert(split_pos, &extra_nodes[&number]);
            }
        }
        // Create the calculation elements by split set positions
        let mut count = 0;
        let mut prev_split_pos: Option<i64> = None;
        if e_split_set.len() == 0 {
            calc_elements.push(CalculationElement::from(&e, nodes, el_num, calc_settings));
        }
        for split_pos in e_split_set.iter() {
            let mut calc_element = CalculationElement::from(&e, nodes, el_num, calc_settings);
            if count == 0 {
                // Create the first element                
                el_num += 1;
                calc_element.node_end = split_pos.1.number;
                calc_element.length = *split_pos.0 as f64;
                calc_element.releases.e_tx = false;
                calc_element.releases.e_tz = false;
                calc_element.releases.e_ry = false;
            } else if count == e_split_set.len() - 1 {
                // Create the last element
                el_num += 1;
                calc_element.node_start = e.node_end;
                calc_element.length = (split_pos.0 - prev_split_pos.unwrap()) as f64;
                calc_element.releases.s_tx = false;
                calc_element.releases.s_tz = false;
                calc_element.releases.s_ry = false;
            } else {
                // Create the middle element
                el_num += 1;
                calc_element.node_start = split_pos.1.number;
                calc_element.node_end = split_pos.1.number;
                calc_element.length = length - prev_split_pos.unwrap() as f64;
                clear_element_releases(&mut calc_element.releases);
            }
            prev_split_pos = Some(*split_pos.0);
            count += 1;
            calc_elements.push(calc_element);
        }
    }
    (calc_elements, extra_nodes)
}

pub fn clear_element_releases(release: &mut Release) {
    release.s_tx = false;
    release.s_tz = false;
    release.s_ry = false;
    release.e_tx = false;
    release.e_tz = false;
    release.e_ry = false;
}