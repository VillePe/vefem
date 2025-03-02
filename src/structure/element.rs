#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::material;
use crate::material::*;
use crate::profile::Profile;
use crate::structure::node::Node;
use std::collections::HashMap;
use crate::structure::release::Release;

#[derive(Debug, Serialize, Deserialize)]
pub enum MaterialType {
    Concrete(Concrete),
    Steel(Steel),
    Timber(Timber),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    pub number: i32,
    pub node_start: i32,
    pub node_end: i32,
    pub material: MaterialType,
    pub profile: Profile,
    pub releases: Release,
}

impl Element {
    pub fn new(
        number: i32,
        node_start: i32,
        node_end: i32,
        profile: Profile,
        material: MaterialType,
    ) -> Self {
        Self {
            number,
            node_start,
            node_end,
            profile,
            material,
            releases: Release::new(),
        }
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

    pub fn get_elastic_modulus(&self) -> f64 {
        material::get_elastic_modulus(&self.material)
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            number: -1,
            node_start: 1,
            node_end: 2,
            profile: Profile::PolygonProfile(crate::profile::PolygonProfile::new_rectangle("R100x100".to_string(), 100.0, 100.0)),
            material: MaterialType::Steel(Steel::new(210000.0)),
            releases: Release::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::material::*;
    use crate::structure::element::{Element, MaterialType};
    use crate::structure::node::Node;
    use crate::profile::{Profile, CustomProfile};
    use std::collections::HashMap;
    use vputilslib::geometry2d::VpPoint;

    #[test]
    fn element_length() {
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(2, VpPoint::new(0.0, 4000.0)));

        let e1: Element = Element::new(
            1,
            1,
            2,
            Profile::CustomProfile(CustomProfile{
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 200_000_000.0,
                custom_area: 6000.0,
                ..CustomProfile::default()
            }),
            MaterialType::Steel(Steel::new(200.0)),
        );
        assert_eq!(e1.get_length(&nodes), 4000.0);
    }
}
