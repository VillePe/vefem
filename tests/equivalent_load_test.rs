mod common;

#[cfg(test)]
mod equivalent_load_tests {
    use approx::relative_eq;
    use vefem::fem::matrices::get_unknown_translation_eq_loads_rows;
    use vefem::fem::matrices::get_unknown_translation_rows;
    use vefem::loads;
    use vefem::loads::Load;
    use vefem::material::Steel;
    use vefem::structure::element::MaterialType;
    use vefem::structure::Element;
    use vefem::structure::Node;
    use vefem::structure::Profile;
    use std::collections::HashMap;
    use vputilslib::equation_handler::EquationHandler;
    use vputilslib::geometry2d::VpPoint;
    use vefem::fem::equivalent_loads::get_element_g_eq_loads;

    use crate::common;
    use crate::common::get_structure_fem_matriisit_releases;

    #[test]
    fn line_load() {
        let el: Element = Element {
            number: 1,
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
        let mut loads = vec![load];
        let elements = vec![el];
        let calc_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#1 {:?}", result);
        assert!((result[0] - (2e4)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (-13333333.0)).abs() < 1.0);
        assert!((result[3] - (2e4)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (13333333.0)).abs() < 1.0);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#2 {:?}", result);
        assert!((result[0] - (20000.0)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (20000.0)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#3 {:?}", result);
        assert!((result[0] - (19.995e3)).abs() < 1e1);
        assert!((result[1] - (0.0)).abs() < 1e1);
        assert!((result[2] - (-9.4281e6)).abs() < 1e3);
        assert!((result[3] - (20.0e3)).abs() < 1e1);
        assert!((result[4] - (0.0e4)).abs() < 1e1);
        assert!((result[5] - (9.4281e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#4 {:?}", result);
        assert!((result[0] - (2.0e4)).abs() < 1e1);
        assert!((result[1] - (0.0)).abs() < 1e1);
        assert!((result[2] - (-11.547e6)).abs() < 1e3);
        assert!((result[3] - (2.0e4)).abs() < 1e1);
        assert!((result[4] - (0.0)).abs() < 1e1);
        assert!((result[5] - (11.547e6)).abs() < 1e3);

        let load = Load::new_line_load(
            "".to_string(),
            "1".to_string(),
            "1000".to_string(),
            "3500".to_string(),
            "10".to_string(),
            0.0,
        );
        loads[0] = load;
        let calc_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        nodes.get_mut(&2).unwrap().point = VpPoint::new(0.0, 4000.0);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#5 {:?}", result);
        assert!((result[0] - (10.4736e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (-9.7493e6)).abs() < 1.0e2);
        assert!((result[3] - (14.5264e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (11.6048e6)).abs() < 1.0e2);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#6 {:?}", result);
        assert!((result[0] - (10.9375e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (14.0625e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#7{:?}", result);
        assert!((result[0] - (10.7024e3)).abs() < 1e1);
        assert!((result[1] - (0.2288e3)).abs() < 1e1);
        assert!((result[2] - (-6.8938e6)).abs() < 1e3);
        assert!((result[3] - (14.2944e3)).abs() < 1e1);
        assert!((result[4] - (-0.2319e3)).abs() < 1e1);
        assert!((result[5] - (8.2058e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#8 {:?}", result);
        assert!((result[0] - (10.5896e3)).abs() < 1e1);
        assert!((result[1] - (-0.2009e3)).abs() < 1e1);
        assert!((result[2] - (-8.4432e6)).abs() < 1e3);
        assert!((result[3] - (14.41044e3)).abs() < 1e1);
        assert!((result[4] - (0.2009e3)).abs() < 1e1);
        assert!((result[5] - (10.0501e6)).abs() < 1e3);
    }

    #[test]
    fn triangular_load() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: HashMap<i32, Node> = HashMap::new();
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
        let mut loads = vec![load];
        let elements = vec![el];
        let calc_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#1 {:?}", result);
        assert!((result[0] - (14e3)).abs() < 0.1);
        assert!((result[1] - (0e3)).abs() < 0.1);
        assert!((result[2] - (-8e6)).abs() < 1.0);
        assert!((result[3] - (6e3)).abs() < 0.1);
        assert!((result[4] - (0e3)).abs() < 0.1);
        assert!((result[5] - (5.333333e6)).abs() < 1.0);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#2 {:?}", result);
        assert!((result[0] - (13.3333e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (6.6667e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#3 {:?}", result);
        assert!((result[0] - (13.6666e3)).abs() < 1e1);
        assert!((result[1] - (-0.33333e3)).abs() < 1e1);
        assert!((result[2] - (-5.6569e6)).abs() < 1e2);
        assert!((result[3] - (6.3333e3)).abs() < 1e1);
        assert!((result[4] - (0.3333e3)).abs() < 1e1);
        assert!((result[5] - (3.7712e6)).abs() < 1e2);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("#4{:?}", result);
        assert!((result[0] - (13.8246e3)).abs() < 1e1);
        assert!((result[1] - (0.2856e3)).abs() < 1e1);
        assert!((result[2] - (-6.9282e6)).abs() < 1e3);
        assert!((result[3] - (6.1629e3)).abs() < 1e1);
        assert!((result[4] - (-0.2899e3)).abs() < 1e1);
        assert!((result[5] - (4.6188e6)).abs() < 1e3);

        let load = Load::new_triangular_load(
            "".to_string(),
            "1".to_string(),
            "1000".to_string(),
            "3500".to_string(),
            "10".to_string(),
            0.0,
        );
        loads[0] = load;
        let calc_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        nodes.get_mut(&2).unwrap().point = VpPoint::new(0.0, 4000.0);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("{:?}", result);
        assert!((result[0] - (7.0068e3)).abs() < 0.1);
        assert!((result[1] - (0.0e3)).abs() < 0.1);
        assert!((result[2] - (-6.1361e6)).abs() < 1.0e2);
        assert!((result[3] - (5.4931e3)).abs() < 0.1);
        assert!((result[4] - (0.0e3)).abs() < 0.1);
        assert!((result[5] - (5.1921e6)).abs() < 1.0e2);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("{:?}", result);
        assert!((result[0] - (6.7708e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (5.7292e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("{:?}", result);
        assert!((result[0] - (6.8839e3)).abs() < 1e1);
        assert!((result[1] - (-0.1162e3)).abs() < 1e1);
        assert!((result[2] - (-4.3389e6)).abs() < 1e3);
        assert!((result[3] - (5.6099e3)).abs() < 1e1);
        assert!((result[4] - (0.1193e3)).abs() < 1e1);
        assert!((result[5] - (3.6714e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("{:?}", result);
        assert!((result[0] - (6.9392e3)).abs() < 1e1);
        assert!((result[1] - (0.099e3)).abs() < 1e1);
        assert!((result[2] - (-5.3139e6)).abs() < 1e3);
        assert!((result[3] - (5.5476e3)).abs() < 1e1);
        assert!((result[4] - (-0.1036e3)).abs() < 1e1);
        assert!((result[5] - (4.4964e6)).abs() < 1e3);

        // 120°
        loads[0].offset_start = "3500".to_string();
        loads[0].offset_end = "1000".to_string();
        let calc_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let result = vefem::fem::equivalent_loads::get_element_g_eq_loads(
            &elements[0],
            &calc_loads,
            &nodes,
        );
        println!("{:?}", result);
        assert!((result[0] - (3.64176e3)).abs() < 1e1);
        assert!((result[1] - (-0.3042e3)).abs() < 1e1);
        assert!((result[2] - (-3.1292e6)).abs() < 1e3);
        assert!((result[3] - (8.8582e3)).abs() < 1e1);
        assert!((result[4] - (0.3007e3)).abs() < 1e1);
        assert!((result[5] - (5.5536e6)).abs() < 1e3);
    }

    #[test]
    fn t_strain_load() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            profile: Profile::new_rectangle("100x100".to_string(), 100., 100.),
            material: MaterialType::Steel(Steel::new(210000.)),
            ..Element::default()
        };
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_strain_load(
            "".to_string(),
            "1".to_string(),
            "10".to_string(),
        );
        let loads = vec![load];
        let elements = vec![el];
        let calc_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let result = get_element_g_eq_loads(
            &elements[0], &calc_loads, &nodes,
        );
        println!("{:?}", result);
        assert!((result[0] - (0e1)).abs() < 0.1);
        assert!((result[1] - (-5250e3)).abs() < 0.1);
        assert!((result[2] - (0e6)).abs() < 1.0);
        assert!((result[3] - (0e3)).abs() < 0.1);
        assert!((result[4] - (5250e3)).abs() < 0.1);
        assert!((result[5] - (0e6)).abs() < 1.0);
    }

    #[test]
    fn t_thermal_load() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            profile: Profile::new_rectangle("100x100".to_string(), 100., 100.),
            material: MaterialType::Steel(Steel::new(210000.)),
            ..Element::default()
        };
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_thermal_load(
            "".to_string(),
            "1".to_string(),
            "10".to_string(),
        );
        let loads = vec![load];
        let elements = vec![el];
        let calc_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());
        let result = get_element_g_eq_loads(
            &elements[0], &calc_loads, &nodes
        );
        println!("{:?}", result);
        assert!((result[0] - (0e1)).abs() < 0.1);
        assert!((result[1] - (-262.5e3)).abs() < 0.1);
        assert!((result[2] - (0e6)).abs() < 1.0);
        assert!((result[3] - (0e3)).abs() < 0.1);
        assert!((result[4] - (262.5e3)).abs() < 0.1);
        assert!((result[5] - (0e6)).abs() < 1.0);
    }

    #[test]
    fn joined_equivalent_load_fem_matriisit_1() {
        // See theory folders xls file (text is in finnish)

        let (elements, nodes) = common::get_structure_fem_matriisit();
        let loads = common::get_fem_matriisi_loads();
        let calc_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());  

        let joined = vefem::fem::equivalent_loads::create(&elements, &nodes, &calc_loads);
        for i in 0..12 {
            let val = joined[(i, 0)];
            if val.abs() < 0.001 {
                print!("{:>9.0}, ",  joined[(i, 0)]);
            } else {
                print!("{:>9.03e}, ", joined[(i, 0)]);
            }
            println!();
        }

        assert!(relative_eq!(joined[(0, 0)], 2.00e04, max_relative = 0.01));
        assert!((            joined[(1, 0)].round() == 0.0));
        assert!(relative_eq!(joined[(2, 0)], -1.33333333e07, max_relative = 0.01));
        assert!(relative_eq!(joined[(3, 0)], 2.00e04, max_relative = 0.01));
        assert!(relative_eq!(joined[(4, 0)], -3e04, max_relative = 0.01));
        assert!(relative_eq!(joined[(5, 0)], -1.6666667e07, max_relative = 0.01));
        assert!(relative_eq!(joined[(6, 0)], -1.00e04, max_relative = 0.01));
        assert!((            joined[(7, 0)].round() == 0.0));
        assert!(relative_eq!(joined[(8, 0)], 6.666667e06, max_relative = 0.01));
        assert!(relative_eq!(joined[(9, 0)], -1e04, max_relative = 0.01));
        assert!(relative_eq!(joined[(10, 0)], -3e04, max_relative = 0.01));
        assert!(relative_eq!(joined[(11, 0)], 2.333e07, max_relative = 0.01));
    }

    #[test]
    fn joined_equivalent_load_fem_matriisit_releases() {
        // See theory folders xls file (text is in finnish)

        let (elements, nodes) = get_structure_fem_matriisit_releases();
        let loads = common::get_fem_matriisi_loads();
        let calc_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());   

        let joined = vefem::fem::equivalent_loads::create(&elements, &nodes, &calc_loads);

        for i in 0..14 {
            let val = joined[(i,0)];
            if val.abs() < 0.001 {
                print!("{:>9.0}, ",  joined[(i,0)]);
            } else {
                print!("{:>9.02e}, ", joined[(i,0)]);
            }
            println!();
        }

        assert!(relative_eq!(joined[(0, 0)], 2.00e04, max_relative = 0.01));
        assert!((            joined[(1, 0)].round() == 0.0));
        assert!(relative_eq!(joined[(2, 0)], -1.33333333e07, max_relative = 0.01));
        assert!(relative_eq!(joined[(3, 0)], 2.00e04, max_relative = 0.01));
        assert!(relative_eq!(joined[(4, 0)], -3e04, max_relative = 0.01));
        assert!(relative_eq!(joined[(5, 0)], -3.0e07, max_relative = 0.01));
        assert!(relative_eq!(joined[(6, 0)], -1.00e04, max_relative = 0.01));
        assert!((            joined[(7, 0)].round() == 0.0));
        assert!(relative_eq!(joined[(8, 0)], 6.6666667e06, max_relative = 0.01));
        assert!(relative_eq!(joined[(9, 0)], -1e04, max_relative = 0.01));
        assert!(relative_eq!(joined[(10, 0)], -3e04, max_relative = 0.01));
        assert!(relative_eq!(joined[(11, 0)], 3.0e07, max_relative = 0.01));
        assert!(relative_eq!(joined[(12, 0)], 1.33333333e07, max_relative = 0.01));
        assert!(relative_eq!(joined[(13, 0)], -6.666667e06, max_relative = 0.01));
    }

    #[test]
    fn t_get_unknown_translation_rows() {
        
        let (elements, nodes) = get_structure_fem_matriisit_releases();
        let loads = common::get_fem_matriisi_loads();
        let calc_loads = loads::utils::extract_calculation_loads(&elements, &nodes, &loads, &EquationHandler::new());   

        let global_eq_loads_matrix = vefem::fem::equivalent_loads::create(
            &elements, 
            &nodes, 
            &calc_loads
        );
        
        let unknown_translation_rows = get_unknown_translation_rows(&nodes, &global_eq_loads_matrix);        
        let eq_loads_results = get_unknown_translation_eq_loads_rows(&unknown_translation_rows, &global_eq_loads_matrix);

        println!("Equivalent loads matrix:");
        for i in 0..eq_loads_results.nrows() {
            for j in 0..eq_loads_results.ncols() {
                let val = eq_loads_results[(i,j)];
                if val.abs() < 0.001 {
                    print!("{:>9.0}, ",  eq_loads_results[(i,j)]);
                } else {
                    print!("{:>9.02e}, ", eq_loads_results[(i,j)]);
                }
            }
            println!();
        }
    }    
}
