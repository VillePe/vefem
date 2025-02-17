#![allow(dead_code)]

use std::collections::HashMap;

use crate::{loads::load::{self, CalculationLoad}, structure::{Element, Node}};

use super::NodeResults;

pub fn calculate_moment_at(x: f64, element: &Element, nodes: &HashMap<i32, Node>, loads: &Vec<CalculationLoad>, results: &NodeResults) -> f64 {
    let mut moment = 0.0;
    let local_reactions = results.get_elem_local_reactions(element, nodes);

    for load in loads {
        match load.load_type {
            load::CalculationLoadType::Point => {
                if load.offset_start <= x {
                    moment -= load.strength * (x - load.offset_start);
                }
            }
            load::CalculationLoadType::Line => {
                if load.offset_start <= x {
                    let load_length;
                    let offset;
                    if load.offset_end <= x {
                        offset = x-(load.offset_start+(load.offset_end - load.offset_start)/2.0);
                        load_length = load.offset_end - load.offset_start;
                    } else {
                        offset = x-(load.offset_start+(x - load.offset_start)/2.0);
                        load_length = x - load.offset_start;
                    }
                    moment -= load.strength * load_length * offset;
                }
            }
            _ => {}
            
        };
    }
    
    moment += local_reactions[(1, 0)] * x;

    moment
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use approx::relative_eq;
    use vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};

    use crate::{loads::{self, Load}, material::Steel, structure::{element::MaterialType, Element, Node, Profile}};

    use super::calculate_moment_at;

    #[test]
    fn t_calculate_moment_at_pl() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let p_load = Load::new_point_load("Pointload".to_string(), "1".to_string(), "L/2".to_string(), "10000".to_string(), -90.0);
        let loads = vec![p_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment: {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 5e6, epsilon = 1e-6), true);
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment: {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10e6, epsilon = 1e-6), true);
        let mom = calculate_moment_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment: {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 5e6, epsilon = 1e-6), true);        
    }

    #[test]
    fn t_calculate_moment_at_ll() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_line_load("Lineload".to_string(), "1".to_string(), "0".to_string(), 
        "L".to_string(), "10".to_string(), -90.0);
        let loads = vec![l_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment: {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, (20000.0*1000.0-(10.0*1000.0*1000.0/2.0)), epsilon = 1.0), true);
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment: {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10.0*4000f64.powi(2)/8.0, epsilon = 1.0), true);
        let mom = calculate_moment_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment: {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, (20000.0*1000.0-(10.0*1000.0*1000.0/2.0)), epsilon = 1.0), true);
    }
}