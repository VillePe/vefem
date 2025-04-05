#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::material;
use crate::material::*;
use crate::profile::Profile;
use crate::settings::CalculationSettings;
use crate::structure::node::Node;
use std::collections::BTreeMap;
use crate::structure::release::Release;

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
    pub number: i32,
    pub node_start: i32,
    pub node_end: i32,
    pub material: MaterialData,
    pub profile: Profile,
    pub releases: Release,
}

impl Element {
    pub fn new(
        number: i32,
        node_start: i32,
        node_end: i32,
        profile: Profile,
        material: MaterialData,
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
    pub fn get_length(&self, nodes: &BTreeMap<i32, Node>) -> f64 {
        let node_start = &nodes[&(self.node_start)];
        let node_end = &nodes[&(self.node_end)];
        vputilslib::geometry2d::calc_length_between_points(&node_start.point, &node_end.point)
    }

    /// Gets the elements rotation in angles (°)
    pub fn get_rotation(&self, nodes: &BTreeMap<i32, Node>) -> f64 {
        let node_start = &nodes[&(self.node_start)];
        let node_end = &nodes[&(self.node_end)];
        vputilslib::geometry2d::get_angle_from_points(&node_start.point, &node_end.point)
    }

    pub fn get_elastic_modulus(&self) -> f64 {
        material::get_elastic_modulus(self.material.value())
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            number: -1,
            node_start: 1,
            node_end: 2,
            profile: Profile::PolygonProfile(crate::profile::PolygonProfile::new_rectangle("R100x100".to_string(), 100.0, 100.0)),
            material: MaterialData::Steel(Steel::new(210000.0)),
            releases: Release::new(),
        }
    }
}

pub struct CalculationElement<'a> {
    pub calc_el_num: i32,
    pub model_el_num: i32,
    pub node_start: i32,
    pub node_end: i32,
    pub material: &'a MaterialData,
    pub profile: &'a Profile,
    pub releases: Release,
    pub length: f64,
    pub rotation: f64,
    pub profile_area: f64,
    pub elastic_modulus: f64,
    pub major_smoa: f64,
}

impl<'a> CalculationElement<'a> {
    pub fn from(element: &'a Element, structure_nodes: &BTreeMap<i32, Node>, number: i32, calc_settings: &CalculationSettings) -> Self {
        Self {
            calc_el_num: number,
            model_el_num: element.number,
            node_start: element.node_start,
            node_end: element.node_end,
            material: &element.material,
            profile: &element.profile,
            releases: element.releases,
            length: element.get_length(structure_nodes),
            rotation: element.get_rotation(structure_nodes),
            elastic_modulus: element.get_elastic_modulus(),
            profile_area: element.profile.get_area(&element.material, calc_settings),
            major_smoa: element.profile.get_major_second_mom_of_area(
                &element.material, 
                calc_settings
            ),
        }   
    }
}

#[cfg(test)]
mod tests {
    use crate::material::*;
    use crate::structure::element::{Element, MaterialData};
    use crate::structure::node::Node;
    use crate::profile::{Profile, CustomProfile};
    use std::collections::BTreeMap;
    use vputilslib::geometry2d::VpPoint;

    #[test]
    fn element_length() {
        let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
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
            MaterialData::Steel(Steel::new(200.0)),
        );
        assert_eq!(e1.get_length(&nodes), 4000.0);
    }
}
