#[cfg(test)]
mod deflection_tests {
    use std::collections::HashMap;

    use approx::relative_eq;
    use vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};

    use vefem::{loads::{self, Load}, material::Steel, structure::{element::MaterialType, Element, Node, Profile}};

    use vefem::fem::deflection;

    #[test]
    fn t_calculate_deflection_at_pl() {
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
        let defl = deflection::calculate_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(1000): {} mm", defl);
        assert!(relative_eq!(defl, -5.238, epsilon = 0.01));
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(2000): {} mm", defl);
        assert!(relative_eq!(defl, -7.619, epsilon = 0.01));
        let defl = deflection::calculate_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(3000): {} mm", defl);
        assert!(relative_eq!(defl, -5.238, epsilon = 0.01));   
        
        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<-45): {} mm", defl);
        assert!(relative_eq!(defl, -5.387, epsilon = 0.01)); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<45): {} mm", defl);
        assert!(relative_eq!(defl, 5.387, epsilon = 0.01));  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(horizontal): {} mm", defl);
        assert!(relative_eq!(defl, 0.0, epsilon = 0.01));      
    }

    #[test]
    fn t_calculate_deflection_at_pl_at_start() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_free(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_fixed(2, VpPoint::new(0.0, 4000.0))),
        ]);
        let elements = vec![el];
        let p_load = Load::new_point_load("Pointload".to_string(), "1".to_string(), "0".to_string(), "10000".to_string(), 0.0);
        let loads = vec![p_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(0.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(L): {} mm", defl);
        assert!(relative_eq!(defl, -121.9, epsilon = 0.1));
    }

    #[test]
    fn t_calculate_deflection_at_pl_at_end() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_fixed(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_free(2, VpPoint::new(0.0, 4000.0))),
        ]);
        let elements = vec![el];
        let p_load = Load::new_point_load("Pointload".to_string(), "1".to_string(), "L".to_string(), "10e3".to_string(), 0.0);
        let loads = vec![p_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(4000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(L): {} mm", defl);
        assert!(relative_eq!(defl, -121.9, epsilon = 0.1));
    }

    #[test]
    fn t_calculate_deflection_at_rl_at_start() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_free(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_fixed(2, VpPoint::new(0.0, 4000.0))),
        ]);
        let elements = vec![el];
        let p_load = Load::new_rotational_load("RotationalLoad".to_string(), "1".to_string(), "0".to_string(), "10e6".to_string());
        let loads = vec![p_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(0.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(L): {} mm", defl);
        assert!(relative_eq!(defl, -45.7143, epsilon = 0.1));
    }

    #[test]
    fn t_calculate_deflection_at_rl() {
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
        let defl = deflection::calculate_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -0.714, epsilon = 0.01), true);
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.0, epsilon = 0.01), true);
        let defl = deflection::calculate_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.714, epsilon = 0.01), true);   
    }

    #[test]
    fn t_calculate_deflection_at_ll() {
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
        let defl = deflection::calculate_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -13.571, epsilon = 0.01), true);
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -19.048, epsilon = 0.01), true);
        let defl = deflection::calculate_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -13.571, epsilon = 0.01), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, -13.469, epsilon = 0.01), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 13.469, epsilon = 0.01), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(horizontal): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.0, epsilon = 0.01), true);  

        loads[0].offset_start = "500".to_string();
        loads[0].offset_end = "1500".to_string();
        loads[0].rotation = -90.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(slice): {} mm", defl);
        assert_eq!(relative_eq!(defl, -5.119, epsilon = 0.01), true);    
    }

    #[test]
    fn t_calculate_deflection_at_tl_ltr_full() {
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
        let defl = deflection::calculate_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -7.083, epsilon = 0.01), true);
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -9.524, epsilon = 0.01), true);
        let defl = deflection::calculate_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -6.488, epsilon = 0.01), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, -6.734, epsilon = 0.01), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 6.734, epsilon = 0.01), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(horizontal)): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.0, epsilon = 0.01), true);      
    }

    /// Slice where load ends before x
    #[test]
    fn t_calculate_deflection_at_tl_ltr_slice() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_triangular_load("TriangularLoad".to_string(), "1".to_string(), "500".to_string(), 
        "1500".to_string(), "10".to_string(), -90.0);
        let mut loads = vec![l_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -2.208, epsilon = 0.1), true);
        let defl = deflection::calculate_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -1.402, epsilon = 0.1), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, -1.562, epsilon = 0.01), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 1.562, epsilon = 0.01), true);
    }

    #[test]
    fn t_calculate_deflection_at_tl_rtl_full() {
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
        let defl = deflection::calculate_at(1000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -6.488, epsilon = 0.01), true);
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -9.524, epsilon = 0.01), true);
        let defl = deflection::calculate_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -7.083, epsilon = 0.01), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, -6.734, epsilon = 0.01), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 6.734, epsilon = 0.01), true);  

        loads[0].rotation = 0.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(horizontal)): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.0, epsilon = 0.01), true);      
    }

    /// Slice where load ends before x
    #[test]
    fn t_calculate_deflection_at_tl_rtl_slice() {
        let el : Element = Element::new(1, 1, 2, 
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0), 
            MaterialType::Steel(Steel::default())
        );
        let nodes = HashMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_triangular_load("TriangularLoad".to_string(), "1".to_string(), "1500".to_string(), 
        "500".to_string(), "10".to_string(), -90.0);
        let mut loads = vec![l_load];
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -2.911, epsilon = 0.1), true);
        let defl = deflection::calculate_at(3000.0, &elements[0], &nodes, &cacl_loads, &results);
        println!("Deflection(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -1.872, epsilon = 0.1), true);

        loads[0].rotation = -45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, -2.058, epsilon = 0.01), true); 

        loads[0].rotation = 45.0;
        let cacl_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let results = vefem::fem::fem_handler::calculate(&elements, &nodes, &loads, &mut EquationHandler::new());
        let defl = deflection::calculate_at(2000.0, &elements[0], &nodes, &cacl_loads, &results);  
        println!("Deflection(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 2.058, epsilon = 0.01), true);
    }
}