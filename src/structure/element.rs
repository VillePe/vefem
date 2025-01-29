#![allow(dead_code)]

use std::collections::HashMap;
use crate::structure::node::Node;
use crate::material::*;
use crate::structure::profile::Profile;

pub enum Material {
    Concrete(concrete::Concrete),
    Steel(steel::Steel),
    Timber(timber::Timber),
}

pub struct Element {
    pub node_start: i32,
    pub node_end: i32,
    pub material: Material,
    pub profile: Profile,
}

impl Element {
    pub fn new(node_start: i32, node_end: i32, profile: Profile, material: Material) -> Self {
        Self{node_start, node_end, profile, material}
    }

    /// Gets the elements length in millimeters (mm)
    pub fn get_length(&self, nodes: &HashMap<i32, Node>) -> f64 {
        let node_start = &nodes[&(self.node_start)];
        let node_end = &nodes[&(self.node_end)];
        vputilslib::geometry2d::calc_length_between_points(&node_start.point, &node_end.point)
    }

    /// Gets the elements rotation in angles (°)
    pub fn get_rotation(&self, nodes: &HashMap<i32, Node>) -> f64 {
        let node_start = &nodes[&(self.node_start)];
        let node_end = &nodes[&(self.node_end)];
        vputilslib::geometry2d::get_angle_from_points(&node_start.point, &node_end.point)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use vputilslib::geometry2d::VpPoint;
    use crate::material::steel::Steel;
    use crate::structure::element::{Element, Material};
    use crate::structure::node::Node;
    use crate::structure::profile::Profile;

    #[test]
    fn element_length() {
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(2, VpPoint::new(0.0, 4000.0)));

        let e1: Element = Element::new(
            1,
            2,
            Profile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 200_000_000.0,
                custom_area: 6000.0,
                ..Profile::default()
            },
            Material::Steel(Steel::new(200.0)),
        );
        assert_eq!(e1.get_length(&nodes), 4000.0);
    }
}