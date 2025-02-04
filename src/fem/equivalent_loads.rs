use crate::fem::matrices;
use crate::loads;
use crate::loads::load::{Load, LoadType};
use crate::structure::{Element, Node};
use nalgebra::{vector, DMatrix, Point};
use std::collections::HashMap;
use std::ptr::eq;
use vputilslib::equation_handler::EquationHandler;

pub fn get_joined_equivalent_loads(
    elements: &Vec<Element>,
    nodes: &HashMap<i32, Node>,
    loads: &Vec<&Load>,
    equation_handler: EquationHandler,
) -> DMatrix<f64> {
    let dof = 3;
    let matrix_vector: Vec<f64> = vec![0.0; nodes.len() * dof];

    for element in elements {}

    DMatrix::from_row_slice(0, 0, &vec![0.0])
}

/// Creates the equivalent load matrix in global coordinates for given element
pub fn get_element_global_equivalent_loads(
    element: &Element,
    loads: &Vec<&Load>,
    nodes: &HashMap<i32, Node>,
    equation_handler: &mut EquationHandler,
) -> DMatrix<f64> {
    let dof = 3;
    let mut result_vector = DMatrix::<f64>::zeros(dof * 2, 1);
    let mut linked_loads: Vec<&Load> = Vec::new();
    // Gather the loads that are linked to the given element
    for l in loads {
        if loads::utils::load_is_linked(&element, &l) {
            linked_loads.push(l);
        }
    }
    let rot_matrix = matrices::get_element_rotation_matrix(&element, nodes);
    let el_length = element.get_length(nodes);
    let el_rotation = element.get_rotation(nodes);
    equation_handler.add_variable("L", el_length);
    // Iterate through the linked loads and add them to the equivalent load matrix
    for load in linked_loads {
        match load.load_type {
            LoadType::Point => {
                let element_eql_matrix_lc = handle_point_load(
                    el_length,
                    el_rotation,
                    load,
                    equation_handler
                );
                let element_eql_matrix_gl = &rot_matrix * element_eql_matrix_lc;
                result_vector += element_eql_matrix_gl;
            }
            LoadType::Line => {
                let element_eql_matrix_lc = handle_line_load(
                    el_length,
                    el_rotation,
                    load,
                    equation_handler
                );
                let element_eql_matrix_gl = &rot_matrix * element_eql_matrix_lc;
                result_vector += element_eql_matrix_gl;
            }
            LoadType::Triangular => {}
            LoadType::Rotational => {}
            LoadType::Trapezoid => {}
            LoadType::Strain => {}
            LoadType::Temperature => {}
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
fn handle_point_load(
    el_length: f64,
    el_rotation: f64,
    load: &Load,
    equation_handler: &mut EquationHandler,
) -> DMatrix<f64> {

    let load_strength: f64 = equation_handler
        .calculate_formula(load.strength.as_str())
        .unwrap_or(0.0);
    let load_off_start = equation_handler
        .calculate_formula(load.offset_start.as_str())
        .unwrap_or(0.0);
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
    vector[0] = -b / el_length * load_value_x;
    vector[3] = -a / el_length * load_value_x;
    // The Z-axis values
    vector[1] = -b.powi(2) * (3.0 * a + b) / el_length.powi(3) * load_value_z;
    vector[4] = -a.powi(2) * (a + 3.0 * b) / el_length.powi(3) * load_value_z;
    // The rotation about Y-axis values
    vector[2] = -a * b.powi(2) / el_length.powi(2) * load_value_z;
    vector[5] = 1.0 * a.powi(2) * b / el_length.powi(2) * load_value_z;

    DMatrix::from_row_slice(6, 1, &vector)
}

fn handle_moment_load(el_length: f64,
                      el_rotation: f64,
                      load: &Load,
                      equation_handler: &mut EquationHandler,) -> DMatrix<f64> {
    let load_strength: f64 = equation_handler
        .calculate_formula(load.strength.as_str())
        .unwrap_or(0.0);
    let load_off_start = equation_handler
        .calculate_formula(load.offset_start.as_str())
        .unwrap_or(0.0);

    let dof = 3;
    let mut vector = vec![0.0; dof * 2];

    let a = load_off_start;
    let b = el_length - a;

    // The Z-axis values
    vector[1] = 6.0*a*b/el_length.powi(3) * load_strength;
    vector[4] = -6.0*a*b/el_length.powi(3) * load_strength;
    // The rotation about Y-axis values
    vector[2] = b*(2.0*a-b)/el_length.powi(2)*load_strength;
    vector[5] = a*(2.0*b-a)/el_length.powi(2)*load_strength;

    DMatrix::from_row_slice(6, 1, &vector)
}

fn handle_line_load(el_length: f64,
                    el_rotation: f64,
                    load: &Load,
                    equation_handler: &mut EquationHandler) -> DMatrix<f64> {
    let load_strength: f64 = equation_handler
        .calculate_formula(load.strength.as_str())
        .unwrap_or(0.0);
    let load_length = load.get_length(equation_handler);
    let load_rotation = load.rotation;

    let dof = 3;
    // The factors to split load into two components
    let local_x_dir = (load_rotation - el_rotation).to_radians().cos();
    let local_z_dir = (load_rotation - el_rotation).to_radians().sin();
    // Equivalent loads sh = start horizontal, sv = start vertical, sr = start rotational load
    let pload_sh_strength = -load_length / 2.0 * local_x_dir * load_strength;
    let pload_eh_strength = -load_length / 2.0 * local_x_dir * load_strength;
    let pload_sv_strength = -load_length / 2.0 * local_z_dir * load_strength;
    let pload_ev_strength = -load_length / 2.0 * local_z_dir * load_strength;
    let pload_sr_strength = -load_length.powi(2) / 12.0 * local_z_dir * load_strength;
    let pload_er_strength = load_length.powi(2) / 12.0 * local_z_dir * load_strength;

    // If the load is the same length as the element and start offset is zero, just return the equivalent loads
    if (load_length - el_length) < 0.1 && load.offset_start.eq("0") {
        return DMatrix::from_row_slice(6, 1, &vec![
            pload_sh_strength,
            pload_sv_strength,
            pload_sr_strength,
            pload_eh_strength,
            pload_ev_strength,
            pload_er_strength]);
    }

    // Otherwise convert the line load to equivalent point and rotational loads
    let mut vector = DMatrix::<f64>::zeros(dof * 2, 1);
    // The strengths of the equivalent loads that are calculated earlier are all pointing at the opposite directions 
    // (because they are support reactions rather than external loads) so they need to be inverted to emulate external 
    // loads
    let pload_start_hor: Load = get_temp_pl(-pload_sh_strength, load.offset_start.clone(), el_rotation);
    let pload_end_hor: Load = get_temp_pl(-pload_eh_strength, load.offset_end.clone(), el_rotation);

    let pload_start_vert: Load = get_temp_pl(-pload_sv_strength, load.offset_start.clone(), el_rotation + 90.0);
    let pload_end_vert: Load = get_temp_pl(-pload_ev_strength, load.offset_end.clone(), el_rotation + 90.0);

    let pload_start_rot: Load = get_temp_rotational_load(load.offset_start.clone(), -pload_sr_strength);
    let pload_end_rot: Load = get_temp_rotational_load(load.offset_end.clone(), -pload_er_strength);

    vector += handle_point_load(el_length, el_rotation, &pload_start_hor, equation_handler);
    vector += handle_point_load(el_length, el_rotation, &pload_start_vert, equation_handler);
    vector += handle_moment_load(el_length, el_rotation, &pload_start_rot, equation_handler);
    vector += handle_point_load(el_length, el_rotation, &pload_end_hor, equation_handler);
    vector += handle_point_load(el_length, el_rotation, &pload_end_vert, equation_handler);
    vector += handle_moment_load(el_length, el_rotation, &pload_end_rot, equation_handler);

    vector
}

/// Helper method to get temporary point load
fn get_temp_pl(equivalent_strength: f64, end: String, el_rotation: f64) -> Load {
    Load::new_point_load(
        "temp".to_string(),
        "temp".to_string(),
        end,
        equivalent_strength.to_string(),
        el_rotation,
    )
}

/// Helper method to get temporary rotational load
fn get_temp_rotational_load(end: String, equivalent_strength: f64) -> Load {
    Load::new_rotational_load(
        "temp".to_string(),
        "temp".to_string(),
        end,
        equivalent_strength.to_string()
    )
}

#[cfg(test)]
mod tests {
    use crate::fem::equivalent_loads::{handle_line_load, handle_moment_load, handle_point_load};
    use crate::loads::Load;
    use crate::structure::{Element, Node};
    use std::collections::HashMap;
    use vputilslib::equation_handler::EquationHandler;
    use vputilslib::geometry2d::{rotate_point, VpPoint};

    #[test]
    fn t_handle_point_load() {
        let el: Element = Element {
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let mut load = Load::new_point_load(
            "".to_string(),
            "1".to_string(),
            "2000".to_string(),
            "10000".to_string(),
            0.0,
        );
        let mut equation_handler = EquationHandler::new();
        let el_length = el.get_length(&nodes);
        let result = handle_point_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (5000.0)).abs() < 0.1);
        assert!((result[2] - (5e6)).abs() < 0.1);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (5000.0)).abs() < 0.1);
        assert!((result[5] - (-5e6)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = handle_point_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        assert!((result[0] - (-5000.0)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (-5000.0)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = handle_point_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        assert!((result[0] - (-3.536e3)).abs() < 1e1);
        assert!((result[1] - (3.536e3)).abs() < 1e1);
        assert!((result[2] - (3.536e6)).abs() < 1e3);
        assert!((result[3] - (-3.536e3)).abs() < 1e1);
        assert!((result[4] - (3.536e3)).abs() < 1e1);
        assert!((result[5] - (-3.536e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = handle_point_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        assert!((result[0] - (2.5e3)).abs() < 1e1);
        assert!((result[1] - (4.330e3)).abs() < 1e1);
        assert!((result[2] - (4.330e6)).abs() < 1e3);
        assert!((result[3] - (2.5e3)).abs() < 1e1);
        assert!((result[4] - (4.330e3)).abs() < 1e1);
        assert!((result[5] - (-4.330e6)).abs() < 1e3);

        load.rotation = -90.0;
        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = handle_point_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        println!("{:?}", result);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (5000.0)).abs() < 0.1);
        assert!((result[2] - (5e6)).abs() < 0.1);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (5000.0)).abs() < 0.1);
        assert!((result[5] - (-5e6)).abs() < 0.1);
    }

    #[test]
    fn t_handle_moment_load() {
        let el: Element = Element {
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_rotational_load(
            "".to_string(),
            "1".to_string(),
            "2000".to_string(),
            "10000".to_string(),
        );
        let mut equation_handler = EquationHandler::new();
        let el_length = el.get_length(&nodes);
        let el_rotation = el.get_rotation(&nodes);
        let result = handle_moment_load(el_length, el_rotation, &load, &mut equation_handler);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (3.75)).abs() < 0.1);
        assert!((result[2] - (2500.0)).abs() < 0.1);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (-3.75)).abs() < 0.1);
        assert!((result[5] - (2500.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = handle_moment_load(el_length, el_rotation, &load, &mut equation_handler);
        assert!((result[0] - (0.0)).abs() < 1e1);
        assert!((result[1] - (3.75)).abs() < 1e1);
        assert!((result[2] - (2500.0)).abs() < 1e3);
        assert!((result[3] - (0.0)).abs() < 1e1);
        assert!((result[4] - (-3.75)).abs() < 1e1);
        assert!((result[5] - (2500.0)).abs() < 1e3);
    }

    #[test]
    fn t_handle_line_load() {
        let el: Element = Element {
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: HashMap<i32, Node> = HashMap::new();
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
        let mut equation_handler = EquationHandler::new();
        let el_length = el.get_length(&nodes);
        let result = handle_line_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        println!("{:?}", result);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (20000.0)).abs() < 0.1);
        assert!((result[2] - (13333333.0)).abs() < 1.0);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (20000.0)).abs() < 0.1);
        assert!((result[5] - (-13333333.0)).abs() < 1.0);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = handle_line_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        println!("{:?}", result);
        assert!((result[0] - (-20000.0)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (-20000.0)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = handle_line_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        println!("{:?}", result);
        assert!((result[0] - (-14.142e3)).abs() < 1e1);
        assert!((result[1] - (14.142e3)).abs() < 1e1);
        assert!((result[2] - (9.4281e6)).abs() < 1e3);
        assert!((result[3] - (-14.142e3)).abs() < 1e1);
        assert!((result[4] - (14.142e3)).abs() < 1e1);
        assert!((result[5] - (-9.4281e6)).abs() < 1e3);
        
        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = handle_line_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        println!("{:?}", result);
        assert!((result[0] - (1.0e4)).abs() < 1e1);
        assert!((result[1] - (1.7321e4)).abs() < 1e1);
        assert!((result[2] - (11.547e6)).abs() < 1e3);
        assert!((result[3] - (1.0e4)).abs() < 1e1);
        assert!((result[4] - (1.7321e4)).abs() < 1e1);
        assert!((result[5] - (-11.547e6)).abs() < 1e3);

        let load = Load::new_line_load(
            "".to_string(),
            "1".to_string(),
            "1000".to_string(),
            "3500".to_string(),
            "10".to_string(),
            0.0,
        );
        nodes.get_mut(&2).unwrap().point = VpPoint::new(0.0, 4000.0);
        let result = handle_line_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        println!("{:?}", result);
        assert!((result[0] - (0.0)).abs() < 0.1);
        assert!((result[1] - (10.4736e3)).abs() < 0.1);
        assert!((result[2] - (9.7493e6)).abs() < 1.0e2);
        assert!((result[3] - (0.0)).abs() < 0.1);
        assert!((result[4] - (14.5264e3)).abs() < 0.1);
        assert!((result[5] - (-11.6048e6)).abs() < 1.0e2);
        
        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = handle_line_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        println!("{:?}", result);
        assert!((result[0] - (-10.9375e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (-14.0625e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);
        
        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = handle_line_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        assert!((result[0] - (-7.734e3)).abs() < 1e1);
        assert!((result[1] - (7.406e3)).abs() < 1e1);
        assert!((result[2] - (6.8938e6)).abs() < 1e3);
        assert!((result[3] - (-9.9437e3)).abs() < 1e1);
        assert!((result[4] - (10.2717e3)).abs() < 1e1);
        assert!((result[5] - (-8.2058e6)).abs() < 1e3);
        
        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = handle_line_load(el_length, el.get_rotation(&nodes), &load, &mut equation_handler);
        assert!((result[0] - (5.4688e3)).abs() < 1e1);
        assert!((result[1] - (9.0704e3)).abs() < 1e1);
        assert!((result[2] - (8.4432e6)).abs() < 1e3);
        assert!((result[3] - (7.0313e3)).abs() < 1e1);
        assert!((result[4] - (12.5802e3)).abs() < 1e1);
        assert!((result[5] - (-10.0501e6)).abs() < 1e3);
    }
}
