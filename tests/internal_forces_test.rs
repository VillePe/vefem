#[cfg(test)]
mod internal_forces_tests {
    use std::collections::HashMap;

    use approx::relative_eq;
    use vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};

    use vefem::{fem::internal_forces::{calculate_axial_force_at, calculate_shear_at}, loads::{self, Load}, material::Steel, structure::{element::MaterialType, Element, Node, Profile}};

    use vefem::fem::internal_forces::calculate_moment_at;

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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<-45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10e6/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, -10e6/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(horizontal): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 0.0, epsilon = 1.0), true);      
    }

    #[test]
    fn t_calculate_moment_at_rl() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let r_load = Load::new_rotational_load("RotationalLoad".to_string(), "1".to_string(), "L/2".to_string(), "10000000".to_string());
        let loads = vec![r_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(1000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 2.5e6, epsilon = 1.0), true);
        let mom = calculate_moment_at(1999.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(1999): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 2500.0*1999.0, epsilon = 1.0), true);
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(2000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, -5e6, epsilon = 1.0), true);
        let mom = calculate_moment_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Moment(3000): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, -2.5e6, epsilon = 1.0), true);   
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<-45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10.0*4000f64.powi(2)/8.0/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, -10.0*4000f64.powi(2)/8.0/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<-45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10.00e6/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, -10.00e6/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<-45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 10.00e6/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(2000<45): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, -10.00e6/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let mom = calculate_moment_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Moment(horizontal)): {} kNm", mom/1e6);
        assert_eq!(relative_eq!(mom, 0.0, epsilon = 1.0), true);      
    }

    #[test]
    fn t_calculate_shear_at_pl() {
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(1000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 5e3, epsilon = 1.0), true);
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(2000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, -5e3, epsilon = 1.0), true);
        let shear = calculate_shear_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(3000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, -5e3, epsilon = 1.0), true);   
        
        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(2000<-45): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, -5e3/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(2000<45): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 5e3/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(horizontal): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 0.0, epsilon = 1.0), true);      
    }

    #[test]
    fn t_calculate_shear_at_rl() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let r_load = Load::new_rotational_load("RotationalLoad".to_string(), "1".to_string(), "L/2".to_string(), "10000000".to_string());
        let loads = vec![r_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);

        println!("Shear(1000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 2.5e3, epsilon = 1.0), true);
        let shear = calculate_shear_at(1999.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(1999): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 2.5e3, epsilon = 1.0), true);
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(2000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 2.5e3, epsilon = 1.0), true);
        let shear = calculate_shear_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(3000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 2.5e3, epsilon = 1.0), true);   
    }

    #[test]
    fn t_calculate_shear_at_ll() {
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        
        println!("Shear(1000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0-10.0*1.0)*1e3, epsilon = 1.0), true);
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(2000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 0.0, epsilon = 1.0), true);
        let shear = calculate_shear_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(3000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0-10.0*3.0)*1e3, epsilon = 1.0), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(1000<-45): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0-10.0*1.0)*1e3/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(1000<45): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, -(10.0*4.0/2.0-10.0*1.0)*1e3/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(horizontal): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 0.0, epsilon = 1.0), true);      
    }

    #[test]
    fn t_calculate_shear_at_tl_ltr_full() {
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);

        println!("Shear(1000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0*2.0/3.0 - 10.0*(4.0-1.0)/4.0*1.0 - (10.0-10.0*(4.0-1.0)/4.0)*1.0/2.0)*1e3, epsilon = 1.0), true);
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(2000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0*2.0/3.0 - 10.0*(4.0-2.0)/4.0*2.0 - (10.0-10.0*(4.0-2.0)/4.0)*2.0/2.0)*1e3, epsilon = 1.0), true);
        let shear = calculate_shear_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(3000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0*2.0/3.0 - 10.0*(4.0-3.0)/4.0*3.0 - (10.0-10.0*(4.0-3.0)/4.0)*3.0/2.0)*1e3, epsilon = 1.0), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(2000<-45): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0*2.0/3.0 - 10.0*(4.0-2.0)/4.0*2.0 - (10.0-10.0*(4.0-2.0)/4.0)*2.0/2.0)*1e3/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(2000<45): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, -(10.0*4.0/2.0*2.0/3.0 - 10.0*(4.0-2.0)/4.0*2.0 - (10.0-10.0*(4.0-2.0)/4.0)*2.0/2.0)*1e3/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(horizontal)): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 0.0, epsilon = 1.0), true);      
    }

    #[test]
    fn t_calculate_shear_at_tl_rtl_full() {
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
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());

        let shear = calculate_shear_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(1000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0*1.0/3.0 - 10.0*(1.0)/4.0*1.0/2.0)*1e3, epsilon = 1.0), true);
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(2000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0*1.0/3.0 - 10.0*(2.0)/4.0*2.0/2.0)*1e3, epsilon = 1.0), true);
        let shear = calculate_shear_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Shear(3000): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0*1.0/3.0 - 10.0*(3.0)/4.0*3.0/2.0)*1e3, epsilon = 1.0), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(2000<-45): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, (10.0*4.0/2.0*1.0/3.0 - 10.0*(2.0)/4.0*2.0/2.0)*1e3/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(2000<45): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, -(10.0*4.0/2.0*1.0/3.0 - 10.0*(2.0)/4.0*2.0/2.0)*1e3/2f64.sqrt(), epsilon = 1.0), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let shear = calculate_shear_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Shear(horizontal)): {} kN", shear/1e3);
        assert_eq!(relative_eq!(shear, 0.0, epsilon = 1.0), true);      
    }

    #[test]
    fn t_calculate_axial_f_at_pl() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let p_load = Load::new_point_load("Pointload".to_string(), "1".to_string(), "L/2".to_string(), "10000".to_string(), 0.0);
        let mut loads = vec![p_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(1000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, 5e3, epsilon = 1.0), true);
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(2000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, -5e3, epsilon = 1.0), true);
        let axial_f = calculate_axial_force_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(3000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, -5e3, epsilon = 1.0), true);   
        
        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Axial force(2000<-45): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, -5e3/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Axial force(2000<45): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, -5e3/2f64.sqrt(), epsilon = 1.0), true);  
    }

    #[test]
    fn t_calculate_axial_f_at_rl() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let r_load = Load::new_rotational_load("RotationalLoad".to_string(), "1".to_string(), "L/2".to_string(), "10000000".to_string());
        let loads = vec![r_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);

        println!("Axial force(1999): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, 0e3, epsilon = 1.0), true);
        let axial_f = calculate_axial_force_at(1999.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(2000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, 0e3, epsilon = 1.0), true);
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(2001): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, 0e3, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_axial_f_at_ll() {
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
        "L".to_string(), "10".to_string(), 0.0);
        let mut loads = vec![l_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        
        println!("Axial force(1000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0-10.0*1.0)*1e3, epsilon = 1.0), true);
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(2000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, 0.0, epsilon = 1.0), true);
        let axial_f = calculate_axial_force_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(3000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0-10.0*3.0)*1e3, epsilon = 1.0), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Axial force(1000<-45): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0-10.0*1.0)*1e3/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Axial force(1000<45): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0-10.0*1.0)*1e3/2f64.sqrt(), epsilon = 1.0), true);       
    }

    #[test]
    fn t_calculate_axial_f_at_tl_ltr_full() {
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
        "L".to_string(), "10".to_string(), 0.0);
        let mut loads = vec![l_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);

        println!("Axial force(1000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0*2.0/3.0 - 10.0*(4.0-1.0)/4.0*1.0 - (10.0-10.0*(4.0-1.0)/4.0)*1.0/2.0)*1e3, epsilon = 1.0), true);
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(2000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0*2.0/3.0 - 10.0*(4.0-2.0)/4.0*2.0 - (10.0-10.0*(4.0-2.0)/4.0)*2.0/2.0)*1e3, epsilon = 1.0), true);
        let axial_f = calculate_axial_force_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(3000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0*2.0/3.0 - 10.0*(4.0-3.0)/4.0*3.0 - (10.0-10.0*(4.0-3.0)/4.0)*3.0/2.0)*1e3, epsilon = 1.0), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Axial force(2000<-45): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0*2.0/3.0 - 10.0*(4.0-2.0)/4.0*2.0 - (10.0-10.0*(4.0-2.0)/4.0)*2.0/2.0)*1e3/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Axial force(2000<45): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0*2.0/3.0 - 10.0*(4.0-2.0)/4.0*2.0 - (10.0-10.0*(4.0-2.0)/4.0)*2.0/2.0)*1e3/2f64.sqrt(), epsilon = 1.0), true);      
    }

    #[test]
    fn t_calculate_axial_f_at_tl_rtl_full() {
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
        "0".to_string(), "10".to_string(), 0.0);
        let mut loads = vec![l_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());

        let axial_f = calculate_axial_force_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(1000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0*1.0/3.0 - 10.0*(1.0)/4.0*1.0/2.0)*1e3, epsilon = 1.0), true);
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(2000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0*1.0/3.0 - 10.0*(2.0)/4.0*2.0/2.0)*1e3, epsilon = 1.0), true);
        let axial_f = calculate_axial_force_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Axial force(3000): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0*1.0/3.0 - 10.0*(3.0)/4.0*3.0/2.0)*1e3, epsilon = 1.0), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Axial force(2000<-45): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0*1.0/3.0 - 10.0*(2.0)/4.0*2.0/2.0)*1e3/2f64.sqrt(), epsilon = 1.0), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let axial_f = calculate_axial_force_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Axial force(2000<45): {} kN", axial_f/1e3);
        assert_eq!(relative_eq!(axial_f, (10.0*4.0/2.0*1.0/3.0 - 10.0*(2.0)/4.0*2.0/2.0)*1e3/2f64.sqrt(), epsilon = 1.0), true);    
    }
}