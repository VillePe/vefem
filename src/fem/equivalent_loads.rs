use crate::fem::matrices;
use crate::loads::load::{CalculationLoad, CalculationLoadType};
use crate::settings::CalculationSettings;
use crate::structure::{Element, Node};
use nalgebra::DMatrix;
use std::collections::BTreeMap;

pub fn create(
    elements: &Vec<Element>,
    nodes: &BTreeMap<i32, Node>,
    loads: &Vec<CalculationLoad>,
    settings: &CalculationSettings,
) -> DMatrix<f64> {
    let supp_count = nodes.len();
    // Increase the joined stiffness matrix size by release count. Releases are set into their
    // own rows and columns at the end of the joined matrix
    let release_count = crate::structure::utils::get_element_release_count(&elements);
    // The degrees of freedom count of single node (tx, tz, ry)
    let dof = 3;
    let col_height = supp_count * dof + release_count;

    let mut matrix_vector = vec![0.0; col_height];

    // The starting column locations for locating the cells for releases
    let mut rel_col = supp_count * dof;
    let mut supp_index: usize;
    let mut i_normalized: usize;

    for elem in elements {
        let el_global_eq_loads = get_element_g_eq_loads(&elem, loads, nodes, settings);
        // The index of the start node
        let s = (elem.node_start - 1) as usize;
        // The index of the end node
        let e = (elem.node_end - 1) as usize;
        for i in 0..dof * 2 {
            if i < dof {
                supp_index = s;
                i_normalized = i;
            } else {
                supp_index = e;
                i_normalized = i - dof;
            }
            // If there is a release at i, it needs to be handled
            if elem.releases.get_release_value(i).unwrap() {
                // If the current row has a release set the current value in the release rows (at the end of the matrix)
                matrix_vector[rel_col] += el_global_eq_loads[(i, 0)];
                rel_col += 1;
            } else {
                // supp_index2 * dof     offset the columns by the support number
                // i_normalized          offset the columns by i
                matrix_vector[supp_index * dof + i_normalized] += el_global_eq_loads[(i, 0)];
            }
        }
    }

    DMatrix::from_vec(col_height, 1, matrix_vector)
}

/// Creates the equivalent load matrix in global coordinates for given element
/// The returned matrix is in the size of \[6 rows, 1 columns] (a column vector)
pub fn get_element_g_eq_loads(
    element: &Element,
    loads: &Vec<CalculationLoad>,
    nodes: &BTreeMap<i32, Node>,
    settings: &CalculationSettings,
) -> DMatrix<f64> {
    let dof = 3;
    let mut result_vector = DMatrix::<f64>::zeros(dof * 2, 1);
    let mut linked_loads: Vec<&CalculationLoad> = Vec::new();
    // Gather the loads that are linked to the given element
    for l in loads {
        if l.element_number == element.number {
            linked_loads.push(l);
        }
    }
    let rot_matrix = matrices::get_element_rotation_matrix(&element, nodes).transpose();
    let el_length = element.get_length(nodes);
    let el_rotation = element.get_rotation(nodes);
    // Iterate through the linked loads and add them to the equivalent load matrix
    for load in linked_loads {
        match load.load_type {
            CalculationLoadType::Point => {
                let element_eql_matrix_lc = handle_point_load(el_length, el_rotation, load);
                let element_eql_matrix_gl = &rot_matrix * element_eql_matrix_lc;
                result_vector += element_eql_matrix_gl;
            }
            CalculationLoadType::Line => {
                let element_eql_matrix_lc = handle_line_load(el_length, el_rotation, load);
                let element_eql_matrix_gl = &rot_matrix * element_eql_matrix_lc;
                result_vector += element_eql_matrix_gl;
            }
            CalculationLoadType::Triangular => {
                let element_eql_matrix_lc = handle_triangular_load(el_length, el_rotation, load);
                let element_eql_matrix_gl = &rot_matrix * element_eql_matrix_lc;
                result_vector += element_eql_matrix_gl;
            }
            CalculationLoadType::Rotational => {
                let element_eql_matrix_lc = handle_rotational_load(el_length, load);
                let element_eql_matrix_gl = &rot_matrix * element_eql_matrix_lc;
                result_vector += element_eql_matrix_gl;
            }
            CalculationLoadType::Strain => {
                let val = element.get_elastic_modulus() * element.profile.get_area(&element.material, settings) / el_length
                    * load.strength;
                result_vector +=
                    &rot_matrix * DMatrix::from_row_slice(6, 1, &[-val, 0.0, 0.0, val, 0.0, 0.0]);
            }
        }
    }

    result_vector
}

/// Handles the conversion of the load to the equivalent loads by elements coordinate system.
/// The returned value is in elements local coordinate system. Equation handler needs to be initialized
/// with needed variables beforehand (element length is not added as a variable here)
///
/// Returns a vector with size of 6 rows and 1 column.
/// - 0: X-axis equivalent loads at the start of the element
/// - 1: Z-axis equivalent loads at the start of the element
/// - 2: rotation about Y-axis equivalent loads at the start of the element
/// - 3: X-axis equivalent loads at the end of the element
/// - 4: Z-axis equivalent loads at the end of the element
/// - 5: rotation about Y-axis equivalent loads at the end of the element
fn handle_point_load(el_length: f64, el_rotation: f64, load: &CalculationLoad) -> DMatrix<f64> {
    let load_strength: f64 = load.strength;
    let load_off_start = load.offset_start;
    let load_rotation = load.rotation;

    let dof = 3;
    let mut vector = vec![0.0; dof * 2];
    let local_x_dir = (load_rotation - el_rotation).to_radians().cos();
    let local_z_dir = (load_rotation - el_rotation).to_radians().sin();

    let load_value_x = local_x_dir * load_strength;
    let load_value_z = local_z_dir * load_strength;

    let a = load_off_start;
    let b = el_length - a;

    // The X-axis values
    vector[0] = b / el_length * load_value_x;
    vector[3] = a / el_length * load_value_x;
    // The Z-axis values
    vector[1] = 1.0 * b.powi(2) * (3.0 * a + b) / el_length.powi(3) * load_value_z;
    vector[4] = 1.0 * a.powi(2) * (a + 3.0 * b) / el_length.powi(3) * load_value_z;
    // The rotation about Y-axis values
    vector[2] = a * b.powi(2) / el_length.powi(2) * load_value_z;
    vector[5] = -1.0 * a.powi(2) * b / el_length.powi(2) * load_value_z;

    DMatrix::from_row_slice(6, 1, &vector)
}

fn handle_rotational_load(el_length: f64, load: &CalculationLoad) -> DMatrix<f64> {
    let load_strength: f64 = load.strength;
    let load_off_start = load.offset_start;

    let dof = 3;
    let mut vector = vec![0.0; dof * 2];

    let a = load_off_start;
    let b = el_length - a;

    // The Z-axis values
    vector[1] = -6.0 * a * b / el_length.powi(3) * load_strength;
    vector[4] = 6.0 * a * b / el_length.powi(3) * load_strength;
    // The rotation about Y-axis values
    vector[2] = -b * (2.0 * a - b) / el_length.powi(2) * load_strength;
    vector[5] = -a * (2.0 * b - a) / el_length.powi(2) * load_strength;

    DMatrix::from_row_slice(6, 1, &vector)
}

fn handle_line_load(el_length: f64, el_rotation: f64, load: &CalculationLoad) -> DMatrix<f64> {
    let load_strength: f64 = load.strength;
    let load_length = load.offset_end - load.offset_start;
    let load_rotation = load.rotation;

    // The factors to split load into two components
    let local_x_dir = (load_rotation - el_rotation).to_radians().cos();
    let local_z_dir = (load_rotation - el_rotation).to_radians().sin();
    // Equivalent loads sh = start horizontal, sv = start vertical, sr = start rotational load
    let pl_sh_strength = load_length / 2.0 * local_x_dir * load_strength;
    let pl_eh_strength = load_length / 2.0 * local_x_dir * load_strength;
    let pl_sv_strength = load_length / 2.0 * local_z_dir * load_strength;
    let pl_ev_strength = load_length / 2.0 * local_z_dir * load_strength;
    let rl_start_strength = load_length.powi(2) / 12.0 * load_strength * local_z_dir;
    let rl_end_strength = -load_length.powi(2) / 12.0 * load_strength * local_z_dir;

    // If the load is the same length as the element and start offset is zero, just return the equivalent loads
    if (load_length - el_length) < 0.1 && load.offset_start == 0.0 {
        return DMatrix::from_row_slice(
            6,
            1,
            &vec![
                pl_sh_strength,
                pl_sv_strength,
                rl_start_strength,
                pl_eh_strength,
                pl_ev_strength,
                rl_end_strength,
            ],
        );
    }

    get_eq_loads_with_partial_eq_loads(
        load,
        el_length,
        el_rotation,
        pl_sh_strength,
        pl_eh_strength,
        pl_sv_strength,
        pl_ev_strength,
        rl_start_strength,
        rl_end_strength,
        false,
    )
}

fn handle_triangular_load(
    el_length: f64,
    el_rotation: f64,
    load: &CalculationLoad,
) -> DMatrix<f64> {
    let load_strength: f64 = load.strength;
    let load_length = load.get_length();
    let load_rotation = load.rotation;

    // The factors to split load into two components
    let local_x_dir = (load_rotation - el_rotation).to_radians().cos();
    let local_z_dir = (load_rotation - el_rotation).to_radians().sin();
    // Equivalent loads sh = start horizontal, sv = start vertical, sr = start rotational load
    let l_offset_start = load.offset_start;
    let l_offset_end = load.offset_end;
    let pl_sh_strength;
    let pl_eh_strength;
    let pl_sv_strength;
    let pl_ev_strength;
    let rl_start_strength;
    let rl_end_strength;
    if l_offset_start < l_offset_end {
        pl_sh_strength = load_length * 2.0 / (2.0 * 3.0) * local_x_dir * load_strength;
        pl_eh_strength = load_length * 1.0 / (2.0 * 3.0) * local_x_dir * load_strength;
        pl_sv_strength = 7.0 * load_length / 20.0 * local_z_dir * load_strength;
        pl_ev_strength = 3.0 * load_length / 20.0 * local_z_dir * load_strength;
        rl_start_strength = load_length.powi(2) / 20.0 * local_z_dir * load_strength;
        rl_end_strength = -load_length.powi(2) / 30.0 * local_z_dir * load_strength;
    } else {
        pl_sh_strength = load_length * 1.0 / (2.0 * 3.0) * local_x_dir * load_strength;
        pl_eh_strength = load_length * 2.0 / (2.0 * 3.0) * local_x_dir * load_strength;
        pl_sv_strength = 3.0 * load_length / 20.0 * local_z_dir * load_strength;
        pl_ev_strength = 7.0 * load_length / 20.0 * local_z_dir * load_strength;
        rl_start_strength = load_length.powi(2) / 30.0 * local_z_dir * load_strength;
        rl_end_strength = -load_length.powi(2) / 20.0 * local_z_dir * load_strength;
    }

    // If the load is the same length as the element and start offset is zero, just return the equivalent loads
    if (load_length - el_length) < 0.1 && load.offset_start == 0.0 {
        return DMatrix::from_row_slice(
            6,
            1,
            &vec![
                pl_sh_strength,
                pl_sv_strength,
                rl_start_strength,
                pl_eh_strength,
                pl_ev_strength,
                rl_end_strength,
            ],
        );
    }

    get_eq_loads_with_partial_eq_loads(
        load,
        el_length,
        el_rotation,
        pl_sh_strength,
        pl_eh_strength,
        pl_sv_strength,
        pl_ev_strength,
        rl_start_strength,
        rl_end_strength,
        l_offset_start > l_offset_end,
    )
}

/// Gets the equivalent loads for the element from equivalent loads that are not set to full length of the element by
/// creating emulating point and rotational loads. The parameter loads are calculated as equivalent loads in the
/// loads length.
fn get_eq_loads_with_partial_eq_loads(
    load: &CalculationLoad,
    el_length: f64,
    el_rotation: f64,
    pl_sh_strength: f64,
    pl_eh_strength: f64,
    pl_sv_strength: f64,
    pl_ev_strength: f64,
    rl_start_strength: f64,
    rl_end_strength: f64,
    swap_offsets: bool,
) -> DMatrix<f64> {
    let dof = 3;
    // Otherwise convert the line load to equivalent point and rotational loads
    let mut vector = DMatrix::<f64>::zeros(dof * 2, 1);
    let offset_start = if swap_offsets {
        load.offset_end
    } else {
        load.offset_start
    };
    let offset_end = if swap_offsets {
        load.offset_start.clone()
    } else {
        load.offset_end.clone()
    };

    let pl_start_hor: CalculationLoad = get_temp_pl(pl_sh_strength, offset_start, el_rotation);
    let pl_end_hor: CalculationLoad = get_temp_pl(pl_eh_strength, offset_end, el_rotation);

    let pl_start_vert: CalculationLoad =
        get_temp_pl(pl_sv_strength, offset_start, el_rotation + 90.0);
    let pl_end_vert: CalculationLoad = get_temp_pl(pl_ev_strength, offset_end, el_rotation + 90.0);

    let rl_start: CalculationLoad = get_temp_rotational_load(offset_start, rl_start_strength);
    let rl_end: CalculationLoad = get_temp_rotational_load(offset_end, rl_end_strength);

    vector += handle_point_load(el_length, el_rotation, &pl_start_hor);
    vector += handle_point_load(el_length, el_rotation, &pl_start_vert);
    vector += handle_rotational_load(el_length, &rl_start);
    vector += handle_point_load(el_length, el_rotation, &pl_end_hor);
    vector += handle_point_load(el_length, el_rotation, &pl_end_vert);
    vector += handle_rotational_load(el_length, &rl_end);

    vector
}

/// Helper method to get temporary point load
fn get_temp_pl(equivalent_strength: f64, end: f64, el_rotation: f64) -> CalculationLoad {
    CalculationLoad {
        name: "NA".to_string(),
        element_number: -1,
        offset_start: end,
        offset_end: end,
        strength: equivalent_strength,
        rotation: el_rotation,
        load_type: CalculationLoadType::Point,
    }
}

/// Helper method to get temporary rotational load
fn get_temp_rotational_load(end: f64, equivalent_strength: f64) -> CalculationLoad {
    CalculationLoad {
        name: "NA".to_string(),
        element_number: -1,
        offset_start: end,
        offset_end: end,
        strength: equivalent_strength,
        rotation: 0.0,
        load_type: CalculationLoadType::Rotational,
    }
}

#[cfg(test)]
mod tests {
    use crate::fem::equivalent_loads::{
        handle_line_load, handle_point_load, handle_rotational_load, handle_triangular_load,
    };
    use crate::loads::{self, Load, LoadCombination};
    use crate::structure::{Element, Node};
    use std::collections::BTreeMap;
    use std::vec;
    use vputilslib::equation_handler::EquationHandler;
    use vputilslib::geometry2d::VpPoint;

    #[test]
    fn t_handle_point_load() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_point_load(
            "".to_string(),
            "1".to_string(),
            "2000".to_string(),
            "10".to_string(),
            0.0,
        );
        let elements = vec![el];
        let loads = &mut vec![load];
        let calc_loads = loads::utils::extract_calculation_loads(
            &elements,
            &nodes,
            loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        let el_length = elements[0].get_length(&nodes);
        let result = handle_point_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (-5000.0)).abs() < 0.1);
        assert!((result[2] - (-5e6)).abs() < 0.1);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (-5000.0)).abs() < 0.1);
        assert!((result[5] - (5e6)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = handle_point_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (5000.0)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (5000.0)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = handle_point_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("{:?}", result);
        assert!((result[0] - (3.536e3)).abs() < 1e1);
        assert!((result[1] - (-3.536e3)).abs() < 1e1);
        assert!((result[2] - (-3.536e6)).abs() < 1e3);
        assert!((result[3] - (3.536e3)).abs() < 1e1);
        assert!((result[4] - (-3.536e3)).abs() < 1e1);
        assert!((result[5] - (3.536e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = handle_point_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (-2.5e3)).abs() < 1e1);
        assert!((result[1] - (-4.330e3)).abs() < 1e1);
        assert!((result[2] - (-4.330e6)).abs() < 1e3);
        assert!((result[3] - (-2.5e3)).abs() < 1e1);
        assert!((result[4] - (-4.330e3)).abs() < 1e1);
        assert!((result[5] - (4.330e6)).abs() < 1e3);

        loads[0].rotation = -90.0;
        let calc_loads = loads::utils::extract_calculation_loads(
            &elements,
            &nodes,
            loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = handle_point_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("{:?}", result);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (-5000.0)).abs() < 0.1);
        assert!((result[2] - (-5e6)).abs() < 0.1);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (-5000.0)).abs() < 0.1);
        assert!((result[5] - (5e6)).abs() < 0.1);
    }

    #[test]
    fn t_handle_point_load_at_start() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_point_load(
            "".to_string(),
            "1".to_string(),
            "0".to_string(),
            "10".to_string(),
            0.0,
        );
        let elements = vec![el];
        let loads = &mut vec![load];
        let calc_loads = loads::utils::extract_calculation_loads(
            &elements,
            &nodes,
            loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        let el_length = elements[0].get_length(&nodes);
        let result = handle_point_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (-10000.0)).abs() < 0.1);
        assert!((result[2] - (-0e6)).abs() < 0.1);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (-0.0)).abs() < 0.1);
        assert!((result[5] - (0e6)).abs() < 0.1);
    }

    #[test]
    fn t_handle_point_load_at_end() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_point_load(
            "".to_string(),
            "1".to_string(),
            "L".to_string(),
            "10".to_string(),
            0.0,
        );
        let elements = vec![el];
        let loads = &mut vec![load];
        let calc_loads = loads::utils::extract_calculation_loads(
            &elements,
            &nodes,
            loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        let el_length = elements[0].get_length(&nodes);
        let result = handle_point_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (-00000.0)).abs() < 0.1);
        assert!((result[2] - (-0e6)).abs() < 0.1);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (-10000.0)).abs() < 0.1);
        assert!((result[5] - (0e6)).abs() < 0.1);
    }

    #[test]
    fn t_handle_moment_load() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_rotational_load(
            "".to_string(),
            "1".to_string(),
            "2000".to_string(),
            "10".to_string(),
        );
        let elements = vec![el];
        let loads = &mut vec![load];
        let calc_loads = loads::utils::extract_calculation_loads(
            &elements,
            &nodes,
            loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        let el_length = elements[0].get_length(&nodes);
        let result = handle_rotational_load(el_length, &calc_loads[0]);
        println!("#1 {:?}", result);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (-3.75e3)).abs() < 0.1);
        assert!((result[2] - (-2500e3)).abs() < 0.1);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (3.75e3)).abs() < 0.1);
        assert!((result[5] - (-2500e3)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = handle_rotational_load(el_length, &calc_loads[0]);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (-3.75e3)).abs() < 0.1);
        assert!((result[2] - (-2500.0e3)).abs() < 0.1);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (3.75e3)).abs() < 0.1);
        assert!((result[5] - (-2500.0e3)).abs() < 0.1);
    }

    #[test]
    fn t_handle_line_load() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_line_load(
            "".to_string(),
            "1".to_string(),
            "0".to_string(),
            "4000".to_string(),
            "10".to_string(),
            -00.0,
        );
        let elements = vec![el];
        let loads = &mut vec![load];
        let calc_loads = loads::utils::extract_calculation_loads(
            &elements,
            &nodes,
            loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        let el_length = elements[0].get_length(&nodes);
        let result = handle_line_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("#1 {:?}", result);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (-20000.0)).abs() < 0.1);
        assert!((result[2] - (-13333333.0)).abs() < 1.0);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (-20000.0)).abs() < 0.1);
        assert!((result[5] - (13333333.0)).abs() < 1.0);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = handle_line_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("#2 {:?}", result);
        assert!((result[0] - (20000.0)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (20000.0)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = handle_line_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("#3 {:?}", result);
        assert!((result[0] - (14.142e3)).abs() < 1e1);
        assert!((result[1] - (-14.142e3)).abs() < 1e1);
        assert!((result[2] - (-9.4281e6)).abs() < 1e3);
        assert!((result[3] - (14.142e3)).abs() < 1e1);
        assert!((result[4] - (-14.142e3)).abs() < 1e1);
        assert!((result[5] - (9.4281e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = handle_line_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("#4 {:?}", result);
        assert!((result[0] - (-1.0e4)).abs() < 1e1);
        assert!((result[1] - (-1.7321e4)).abs() < 1e1);
        assert!((result[2] - (-11.547e6)).abs() < 1e3);
        assert!((result[3] - (-1.0e4)).abs() < 1e1);
        assert!((result[4] - (-1.7321e4)).abs() < 1e1);
        assert!((result[5] - (11.547e6)).abs() < 1e3);

        loads[0] = Load::new_line_load(
            "".to_string(),
            "1".to_string(),
            "1000".to_string(),
            "3500".to_string(),
            "10".to_string(),
            0.0,
        );
        let calc_loads = loads::utils::extract_calculation_loads(
            &elements,
            &nodes,
            loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        nodes.get_mut(&2).unwrap().point = VpPoint::new(0.0, 4000.0);
        let result = handle_line_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("#5 {:?}", result);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (-10.4736e3)).abs() < 0.1);
        assert!((result[2] - (-9.7493e6)).abs() < 1.0e2);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (-14.5264e3)).abs() < 0.1);
        assert!((result[5] - (11.6048e6)).abs() < 1.0e2);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = handle_line_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("#6 {:?}", result);
        assert!((result[0] - (10.9375e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (14.0625e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = handle_line_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (7.734e3)).abs() < 1e1);
        assert!((result[1] - (-7.406e3)).abs() < 1e1);
        assert!((result[2] - (-6.8938e6)).abs() < 1e3);
        assert!((result[3] - (9.9437e3)).abs() < 1e1);
        assert!((result[4] - (-10.2717e3)).abs() < 1e1);
        assert!((result[5] - (8.2058e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = handle_line_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (-5.4688e3)).abs() < 1e1);
        assert!((result[1] - (-9.0704e3)).abs() < 1e1);
        assert!((result[2] - (-8.4432e6)).abs() < 1e3);
        assert!((result[3] - (-7.0313e3)).abs() < 1e1);
        assert!((result[4] - (-12.5802e3)).abs() < 1e1);
        assert!((result[5] - (10.0501e6)).abs() < 1e3);
    }

    #[test]
    fn t_handle_triangular_load() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_triangular_load(
            "".to_string(),
            "1".to_string(),
            "0".to_string(),
            "4000".to_string(),
            "10".to_string(),
            -00.0,
        );
        let elements = vec![el];
        let loads = &mut vec![load];
        let calc_loads = loads::utils::extract_calculation_loads(
            &elements,
            &nodes,
            loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        let el_length = elements[0].get_length(&nodes);
        let result =
            handle_triangular_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (-14e3)).abs() < 0.1);
        assert!((result[2] - (-8e6)).abs() < 1.0);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (-6e3)).abs() < 0.1);
        assert!((result[5] - (5.333333e6)).abs() < 1.0);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result =
            handle_triangular_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (13.3333e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (6.6667e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result =
            handle_triangular_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (9.4281e3)).abs() < 1e1);
        assert!((result[1] - (-9.8995e3)).abs() < 1e1);
        assert!((result[2] - (-5.6569e6)).abs() < 1e2);
        assert!((result[3] - (4.714e3)).abs() < 1e1);
        assert!((result[4] - (-4.2426e3)).abs() < 1e1);
        assert!((result[5] - (3.7712e6)).abs() < 1e2);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result =
            handle_triangular_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (-6.6666e3)).abs() < 1e1);
        assert!((result[1] - (-12.1244e3)).abs() < 1e1);
        assert!((result[2] - (-6.9282e6)).abs() < 1e3);
        assert!((result[3] - (-3.3333e3)).abs() < 1e1);
        assert!((result[4] - (-5.1962e3)).abs() < 1e1);
        assert!((result[5] - (4.6188e6)).abs() < 1e3);

        loads[0] = Load::new_line_load(
            "".to_string(),
            "1".to_string(),
            "1000".to_string(),
            "3500".to_string(),
            "10".to_string(),
            0.0,
        );
        let calc_loads = loads::utils::extract_calculation_loads(
            &elements,
            &nodes,
            loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        nodes.get_mut(&2).unwrap().point = VpPoint::new(0.0, 4000.0);
        let result =
            handle_triangular_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("{:?}", result);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (-7.0068e3)).abs() < 0.1);
        assert!((result[2] - (-6.1361e6)).abs() < 1.0e2);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (-5.4932e3)).abs() < 0.1);
        assert!((result[5] - (5.1921e6)).abs() < 1.0e2);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result =
            handle_triangular_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("{:?}", result);
        assert!((result[0] - (6.7708e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (5.7292e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result =
            handle_triangular_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        assert!((result[0] - (4.7877e3)).abs() < 1e1);
        assert!((result[1] - (-4.9546e3)).abs() < 1e1);
        assert!((result[2] - (-4.3389e6)).abs() < 1e3);
        assert!((result[3] - (4.0511e3)).abs() < 1e1);
        assert!((result[4] - (-3.8843e3)).abs() < 1e1);
        assert!((result[5] - (3.6713e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result =
            handle_triangular_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("{:?}", result);
        assert!((result[0] - (-3.3854e3)).abs() < 1e1);
        assert!((result[1] - (-6.0681e3)).abs() < 1e1);
        assert!((result[2] - (-5.314e6)).abs() < 1e3);
        assert!((result[3] - (-2.8646e3)).abs() < 1e1);
        assert!((result[4] - (-4.7572e3)).abs() < 1e1);
        assert!((result[5] - (4.4965e6)).abs() < 1e3);

        // 120°
        loads[0].offset_start = "3500".to_string();
        loads[0].offset_end = "1000".to_string();
        let calc_loads = loads::utils::extract_calculation_loads(
            &elements,
            &nodes,
            loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        let result =
            handle_triangular_load(el_length, elements[0].get_rotation(&nodes), &calc_loads[0]);
        println!("{:?}", result);
        assert!((result[0] - (-2.0833e3)).abs() < 1e1);
        assert!((result[1] - (-3.0029e3)).abs() < 1e1);
        assert!((result[2] - (-3.1292e6)).abs() < 1e3);
        assert!((result[3] - (-4.1667e3)).abs() < 1e1);
        assert!((result[4] - (-7.823e3)).abs() < 1e1);
        assert!((result[5] - (5.5536e6)).abs() < 1e3);
    }
}
