#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};

use vefem::{
    fem::CalcModel,
    loads::{Load, LoadGroup},
    material::{MaterialData, Steel},
    profile::Profile,
    settings::CalculationSettings,
    structure::{Element, Node},
};
use vputilslib::geometry2d::{Polygon, VpPoint};

pub fn get_default_profile() -> Profile {
    Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0)
}

pub fn get_default_material_steel() -> MaterialData {
    MaterialData::Steel(Steel::new_s355())
}

pub fn get_default_line_load(element_numbers: &str) -> Load {
    Load::new_line_load(
        "LineLoad".to_string(),
        element_numbers.to_string(),
        "0".to_string(),
        "L".to_string(),
        "10".to_string(),
        -90.0,
        LoadGroup::PERMANENT
    )
}

pub fn get_structure_fem_matriisit_releases() -> (Vec<Element>, BTreeMap<i32, Node>) {
    let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
    nodes.insert(1, Node::new_fixed(1, VpPoint::new(0.0, 0.0))); // 0, 0
    nodes.insert(2, Node::new_free(2, VpPoint::new(0.0, 4000.0))); // 0, 4000
    nodes.insert(
        3,
        Node::new_fixed(3, VpPoint::new(nodes[&1].point.x + 6000.0, 0.0)),
    ); // 6000, 0
    nodes.insert(
        4,
        Node::new_free(4, VpPoint::new(nodes[&3].point.x, nodes[&2].point.y)),
    ); // 6000, 4000

    let mut e1: Element = Element::new(
        1,
        1,
        2,
        Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
        MaterialData::Steel(Steel::new(210e3)),
    );
    let e2: Element = Element::new(
        2,
        2,
        4,
        Profile::new_rectangle("R200x100".to_string(), 200.0, 100.0),
        MaterialData::Steel(Steel::new(210e3)),
    );
    let mut e3: Element = Element::new(
        3,
        3,
        4,
        Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
        MaterialData::Steel(Steel::new(210e3)),
    );

    e1.releases.e_ry = true;
    e3.releases.e_ry = true;

    let elements = vec![e1, e2, e3];

    (elements, nodes)
}

pub fn get_structure_fem_matriisit() -> (Vec<Element>, BTreeMap<i32, Node>) {
    let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
    nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
    nodes.insert(2, Node::new_free(2, VpPoint::new(0.0, 4000.0)));
    nodes.insert(
        3,
        Node::new_hinged(3, VpPoint::new(nodes[&2].point.x + 6000.0, 0.0)),
    );
    nodes.insert(
        4,
        Node::new_free(4, VpPoint::new(nodes[&3].point.x, nodes[&2].point.y)),
    );

    let e1: Element = Element::new(
        1,
        1,
        2,
        Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
        MaterialData::Steel(Steel::new(210e3)),
    );
    let e2: Element = Element::new(
        2,
        2,
        4,
        Profile::new_rectangle("R200x100".to_string(), 200.0, 100.0),
        MaterialData::Steel(Steel::new(210e3)),
    );
    let e3: Element = Element::new(
        3,
        3,
        4,
        Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
        MaterialData::Steel(Steel::new(210e3)),
    );

    let elements = vec![e1, e2, e3];

    (elements, nodes)
}

pub fn get_structure_three_horizontal_elements() -> (Vec<Element>, BTreeMap<i32, Node>) {
    let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
    nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
    nodes.insert(2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0)));
    nodes.insert(3, Node::new_hinged(3, VpPoint::new(4000.0 + 4000.0, 0.0)));
    nodes.insert(
        4,
        Node::new_hinged(4, VpPoint::new(4000.0 + 4000.0 + 4000.0, 0.0)),
    );

    let e1: Element = Element::new(
        1,
        1,
        2,
        Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
        MaterialData::Steel(Steel::new(210e3)),
    );
    let e2: Element = Element::new(
        2,
        2,
        3,
        Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
        MaterialData::Steel(Steel::new(210e3)),
    );
    let e3: Element = Element::new(
        3,
        3,
        4,
        Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
        MaterialData::Steel(Steel::new(210e3)),
    );

    let elements = vec![e1, e2, e3];

    (elements, nodes)
}

pub fn get_fem_matriisi_loads() -> Vec<Load> {
    let line_load_1 = Load::new_line_load(
        "1".to_string(),
        "1".to_string(),
        "0".to_string(),
        "L".to_string(),
        "10".to_string(),
        0.0,
        LoadGroup::PERMANENT,
    );
    let line_load_2 = Load::new_line_load(
        "2".to_string(),
        "2".to_string(),
        "0".to_string(),
        "L".to_string(),
        "10".to_string(),
        -90.0,
        LoadGroup::PERMANENT,
    );
    let line_load_3 = Load::new_line_load(
        "3".to_string(),
        "3".to_string(),
        "0".to_string(),
        "L".to_string(),
        "5".to_string(),
        180.0,
        LoadGroup::PERMANENT,
    );
    let loads = vec![line_load_1, line_load_2, line_load_3];
    loads
}

pub fn get_inversed_t_profile() -> Profile {
    Profile::new(
        "name".to_string(),
        Polygon::new(vec![
            VpPoint::new(0.0, 0.0),
            VpPoint::new(880.0, 0.0),
            VpPoint::new(880.0, 250.0),
            VpPoint::new(680.0, 250.0),
            VpPoint::new(680.0, 580.0),
            VpPoint::new(200.0, 580.0),
            VpPoint::new(200.0, 250.0),
            VpPoint::new(0.0, 250.0),
            VpPoint::new(0.0, 0.0),
        ]),
    )
}

pub fn get_calc_model<'a>(
    elements: &'a Vec<Element>,
    nodes: &'a BTreeMap<i32, Node>,
) -> CalcModel<'a> {
    let (calc_elements, extra_nodes) = vefem::structure::utils::get_calc_elements(
        elements,
        nodes,
        &HashMap::new(),
        &CalculationSettings::default(),
    );
    CalcModel::new(nodes, extra_nodes, elements, calc_elements)
}
