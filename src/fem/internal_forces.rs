#![allow(dead_code)]

use std::collections::HashMap;

use crate::{loads::load::{self, CalculationLoad}, structure::{Element, Node}};

use super::NodeResults;

pub fn calculate_moment_at(x: f64, element: &Element, nodes: &HashMap<i32, Node>, loads: &Vec<CalculationLoad>, results: &NodeResults) -> f64 {
    let mut moment = 0.0;
    let local_reactions = results.get_elem_local_reactions(element, nodes);

    for load in loads {
        // The factor to handle skewed loads
        let z_dir_factor = load.rotation.to_radians().sin();
        match load.load_type {
            load::CalculationLoadType::Point => {
                if load.offset_start <= x {
                    moment += load.strength * z_dir_factor * (x - load.offset_start);
                }
            }
            load::CalculationLoadType::Rotational => {
                if load.offset_start <= x {
                    moment += load.strength * (x - load.offset_start);
                }
            }
            load::CalculationLoadType::Line => {
                if load.offset_start <= x {
                    let load_length;
                    if load.offset_end <= x {
                        load_length = load.offset_end - load.offset_start;
                    } else {
                        load_length = x - load.offset_start;                      
                    }
                    let offset = x-(load.offset_start+load_length/2.0);
                    moment += load.strength * z_dir_factor * load_length * offset;
                }
            }
            load::CalculationLoadType::Triangular => {
                // Triangular load with max load at left hand side
                if load.offset_start < load.offset_end {
                    moment += moment_triang_ltr(load, x)
                } else {
                    moment += moment_triang_rtl(load, x)
                }
                
            }
            _ => {}
            
        };
    }
    
    moment += local_reactions[(1, 0)] * x;

    moment
}

/// Calculates the moment at x for a triangular load with the maximum load at the left hand side.
/// ltr = Left to right
fn moment_triang_ltr(load: &CalculationLoad, x: f64) -> f64 {
    let mut moment = 0.0;
    let z_dir_factor = load.rotation.to_radians().sin();
    if load.offset_start <= x {
        if load.offset_end <= x {
            let load_length = load.offset_end - load.offset_start;                            
            let offset = x-(load.offset_start+load_length*1.0/3.0);                            
            moment += load.strength * z_dir_factor * load_length / 2.0 * offset;
        } else {
            // Split the load into a line load and a triangular load at x.
            let load_length = x - load.offset_start;  
            let offset_tl = x-(load.offset_start+(load_length)*1.0/3.0);          
            // The minimum strength at x (right hand side of the load)
            let strength_min = 
                load.strength -
                load.strength * 
                (x - load.offset_start) / 
                (load.offset_end - load.offset_start);                    
            let strength_ll = strength_min;
            let strength_tl = load.strength - strength_ll;
            // Moment from triangular load = F * l / 2 * offset
            moment += strength_tl * z_dir_factor * load_length / 2.0 * offset_tl;
            let offset_ll = x-(load.offset_start+(x - load.offset_start)/2.0);
            moment += strength_ll * z_dir_factor * load_length * offset_ll;
        }                        
    }
    moment
}

/// Calculates the moment at x for a triangular load with the maximum load at the right hand side.
/// ltr = Left to right
fn moment_triang_rtl(load: &CalculationLoad, x: f64) -> f64 {
    let z_dir_factor = load.rotation.to_radians().sin();
    // Load offsets at left or right hand side
    let left = load.offset_end;
    let right = load.offset_start;
    if left <= x {
        let load_length;
        let offset;
        if right <= x {
            offset = x-(left+(right - left)*2.0/3.0);
            load_length = right - left;                            
            return load.strength * z_dir_factor * load_length / 2.0 * offset
        } else {
            // Split the load at x. No need to split into a line load, because of 
            // the direction
            let load_length = x - left;  
            let offset_tl = x-(left+(load_length)*2.0/3.0);    
            // The minimum strength at x (right hand side of the load)
            let strength_max = 
                load.strength * 
                (x - left) / 
                (right - left);
            return strength_max * z_dir_factor * load_length / 2.0 * offset_tl
        }                       
    }
    0.0
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
        let mut loads = vec![p_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(1000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 5e6, epsilon = 1.0), true);
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(2000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10e6, epsilon = 1.0), true);
        let mom = calculate_moment_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(3000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 5e6, epsilon = 1.0), true);   
        
        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<-45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10e6/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, -10e6/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(horizontal): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 0.0, epsilon = 1.0), true);      
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
        let mut loads = vec![l_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(1000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, (20000.0*1000.0-(10.0*1000.0*1000.0/2.0)), epsilon = 1.0), true);
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(2000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10.0*4000f64.powi(2)/8.0, epsilon = 1.0), true);
        let mom = calculate_moment_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(3000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, (20000.0*1000.0-(10.0*1000.0*1000.0/2.0)), epsilon = 1.0), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<-45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10.0*4000f64.powi(2)/8.0/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, -10.0*4000f64.powi(2)/8.0/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(horizontal): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 0.0, epsilon = 1.0), true);      
    }

    #[test]
    fn t_calculate_moment_at_tl_ltr_full() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_triangular_load("TriangularLoad".to_string(), "1".to_string(), "0".to_string(), 
        "L".to_string(), "10".to_string(), -90.0);
        let mut loads = vec![l_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(1000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 8.75e6, epsilon = 1.0), true);
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(2000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10.00e6, epsilon = 1.0), true);
        let mom = calculate_moment_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(3000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 6.25e6, epsilon = 1.0), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<-45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10.00e6/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, -10.00e6/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(horizontal)): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 0.0, epsilon = 1.0), true);      
    }

    #[test]
    fn t_calculate_moment_at_tl_rtl_full() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_triangular_load("TriangularLoad".to_string(), "1".to_string(), "L".to_string(), 
        "0".to_string(), "10".to_string(), -90.0);
        let mut loads = vec![l_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(1000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 6.25e6, epsilon = 1.0), true);
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(2000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10.00e6, epsilon = 1.0), true);
        let mom = calculate_moment_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(3000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 8.75e6, epsilon = 1.0), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<-45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10.00e6/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, -10.00e6/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = crate::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(horizontal)): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 0.0, epsilon = 1.0), true);      
    }
}