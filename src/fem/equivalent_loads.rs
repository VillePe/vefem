use std::collections::HashMap;
use crate::loads::load::{Load, LoadType};
use crate::structure::{Element, Node};
use nalgebra::{DMatrix, Point};
use vputilslib::equation_handler::EquationHandler;
use crate::fem::matrices;
use crate::loads;

pub fn get_joined_equivalent_loads(
    elements: &Vec<Element>,
    nodes: &HashMap<i32, Node>,
    loads: &Vec<&Load>,
    equation_handler: EquationHandler,
) -> DMatrix<f64> {
    let dof = 3;
    let matrix_vector : Vec<f64> = vec![0.0; nodes.len() * dof];
    
    for element in elements {
        
    }
    
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
    let mut result_vector = DMatrix::<f64>::zeros(dof*2, 1);
    let mut linked_loads: Vec<&Load> = Vec::new();
    // Gather the loads that are linked to the given element
    for l in loads {
        if loads::utils::load_is_linked(&element, &l) {
            linked_loads.push(l);
        }
    }
    let rot_matrix = matrices::get_element_rotation_matrix(&element, nodes);
    // Iterate through the linked loads and add them to the equivalent load matrix
    for l in linked_loads {
        match l.load_type {
            LoadType::Point => {
                let element_eql_matrix_lc = handle_point_load(element, l, nodes, equation_handler);
                let element_eql_matrix_gl = &rot_matrix * element_eql_matrix_lc;
                result_vector += element_eql_matrix_gl;
            }
            LoadType::Line => {}
            LoadType::Triangular => {}
            LoadType::Moment => {}
            LoadType::Trapezoid => {}
            LoadType::Strain => {}
            LoadType::Temperature => {}
        }
    }

    result_vector
}

/// Handles the conversion of the load to the equivalent loads by elements coordinate system.
/// The returned value is in elements local coordinate system.
/// 
/// Returns a vector with size of 6 rows and 1 column.
/// - 0: X-axis equivalent loads at the start of the element
/// - 1: Z-axis equivalent loads at the start of the element
/// - 2: rotation about Y-axis equivalent loads at the start of the element
/// - 3: X-axis equivalent loads at the end of the element
/// - 4: Z-axis equivalent loads at the end of the element
/// - 5: rotation about Y-axis equivalent loads at the end of the element
fn handle_point_load(element: &Element, load: &Load, nodes: &HashMap<i32, Node>, equation_handler: &mut EquationHandler) -> DMatrix<f64> {
    let dof = 3;
    let mut vector = vec![0.0; dof*2];
    let el_length = element.get_length(nodes);
    let el_rotation = element.get_rotation(nodes);
    let load_rotation_normalized = vputilslib::geometry2d::normalize_angle(load.rotation);
    let local_x_dir = (load_rotation_normalized-el_rotation).to_radians().cos();
    let local_z_dir = (load_rotation_normalized-el_rotation).to_radians().sin();

    equation_handler.add_variable("L", el_length);
    let load_strength = equation_handler.calculate_formula(load.strength.as_str()).unwrap_or(0.0);

    let load_value_x = local_x_dir * load_strength;
    let load_value_z = local_z_dir * load_strength;

    let a = equation_handler.calculate_formula(load.offset_start.as_str()).unwrap_or(0.0);
    let b = el_length - a;

    println!("-{b}/{el_length}*{load_value_x}");
    // The X-axis values
    vector[0] = -b/el_length*load_value_x;
    vector[3] = -a/el_length*load_value_x;
    // The Z-axis values
    vector[1] = b.powi(2)*(3.0*a+b)/el_length.powi(3)*load_value_z;
    vector[4] = a.powi(2)*(a+3.0*b)/el_length.powi(3)*load_value_z;
    // The rotation about Y-axis values
    vector[2] = a*b.powi(2)/el_length.powi(2)*load_value_z;
    vector[5] = -1.0*a.powi(2)*b/el_length.powi(2)*load_value_z;

    DMatrix::from_row_slice(6, 1, &vector)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use vputilslib::equation_handler::EquationHandler;
    use vputilslib::geometry2d::VpPoint;
    use crate::fem::equivalent_loads::handle_point_load;
    use crate::loads::Load;
    use crate::structure::{Element, Node};

    #[test]
    fn t_handle_point_load() {
        let el : Element = Element{node_start: 1, node_end:2, ..Element::default()};
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_point_load("".to_string(), "1".to_string(), "2000".to_string(), "10000".to_string(), 0.0);
        let mut equation_handler = EquationHandler::new();
        let result = handle_point_load(&el, &load, &nodes, &mut equation_handler);
        assert!((result[0]-(0.0)).abs()<0.1);
        assert!((result[1]-(-5000.0)).abs()<0.1);
        assert!((result[2]-(-5000000.0)).abs()<0.1);
        assert!((result[3]-(0.0)).abs()<0.1);
        assert!((result[4]-(-5000.0)).abs()<0.1);
        assert!((result[5]-(5000000.0)).abs()<0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = handle_point_load(&el, &load, &nodes, &mut equation_handler);
        assert!((result[0]-(-5000.0)).abs()<0.1);
        assert!((result[1]-(0.0)).abs()<0.1);
        assert!((result[2]-(0.0)).abs()<0.1);
        assert!((result[3]-(-5000.0)).abs()<0.1);
        assert!((result[4]-(0.0)).abs()<0.1);
        assert!((result[5]-(0.0)).abs()<0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = handle_point_load(&el, &load, &nodes, &mut equation_handler);
        assert!((result[0]-(-3.536e3)).abs()<1e1);
        assert!((result[1]-(-3.536e3)).abs()<1e1);
        assert!((result[2]-(-3.536e6)).abs()<1e3);
        assert!((result[3]-(-3.536e3)).abs()<1e1);
        assert!((result[4]-(-3.536e3)).abs()<1e1);
        assert!((result[5]-(3.536e6)).abs()<1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = handle_point_load(&el, &load, &nodes, &mut equation_handler);
        assert!((result[0]-(2.5e3)).abs()<1e1);
        assert!((result[1]-(-4.330e3)).abs()<1e1);
        assert!((result[2]-(-4.330e6)).abs()<1e3);
        assert!((result[3]-(2.5e3)).abs()<1e1);
        assert!((result[4]-(-4.330e3)).abs()<1e1);
        assert!((result[5]-(4.330e6)).abs()<1e3);
    }
}

