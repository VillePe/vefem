mod common;

#[cfg(test)]
mod tests {
    use approx::relative_eq;
    use idfem::fem::matrices::{
        get_unknown_translation_eq_loads_rows, get_unknown_translation_rows,
        get_unknown_translation_stiffness_rows,
    };
    use idfem::material::Steel;
    use idfem::structure::element::MaterialType;
    use idfem::structure::Node;
    use idfem::structure::Profile;
    use idfem::structure::{element, Element};
    use std::collections::HashMap;
    use vputilslib::geometry2d;
    use vputilslib::geometry2d::VpPoint;

    use crate::common;

    #[test]
    fn displacement_1() {
        let (elements, nodes) = common::get_structure_fem_matriisit();
        let loads = common::get_fem_matriisi_loads();
        let mut equation_handler = vputilslib::equation_handler::EquationHandler::new();
        let gl_stiff_m = idfem::fem::stiffness::create_joined_stiffness_matrix(&elements, &nodes);
        let gl_eq_loads_m = idfem::fem::equivalent_loads::create_joined_equivalent_loads(
            &elements,
            &nodes,
            &loads,
            &mut equation_handler,
        );
        let displacement = idfem::fem::fem_handler::calculate_displacements(
            &nodes,
            idfem::fem::utils::col_height(&nodes, &elements),
            &gl_stiff_m,
            &gl_eq_loads_m,
        );
        println!("{}", displacement);
        assert!(relative_eq!(
            displacement[(0, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(1, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(2, 0)],
            -0.0364,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(3, 0)],
            81.9357,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(4, 0)],
            -0.0444,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(5, 0)],
            -0.00394,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(6, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(7, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(8, 0)],
            -0.0274,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(9, 0)],
            81.9077,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(10, 0)],
            -0.0698,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(11, 0)],
            00.001077,
            max_relative = 0.01
        ));
    }

    #[test]
    fn displacement_2() {
        let (elements, nodes) = common::get_structure_fem_matriisit_releases();
        let loads = common::get_fem_matriisi_loads();
        let mut equation_handler = vputilslib::equation_handler::EquationHandler::new();
        let gl_stiff_m = idfem::fem::stiffness::create_joined_stiffness_matrix(&elements, &nodes);
        let gl_eq_loads_m = idfem::fem::equivalent_loads::create_joined_equivalent_loads(
            &elements,
            &nodes,
            &loads,
            &mut equation_handler,
        );
        let displacement = idfem::fem::fem_handler::calculate_displacements(
            &nodes,
            idfem::fem::utils::col_height(&nodes, &elements),
            &gl_stiff_m,
            &gl_eq_loads_m,
        );
        println!("{}", displacement);
        assert!(relative_eq!(
            displacement[(0, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(1, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(2, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(3, 0)],
            45.7223,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(4, 0)],
            -0.0571,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(5, 0)],
            -0.00643,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(6, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(7, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(8, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(9, 0)],
            45.7063,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(10, 0)],
            -0.0571,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(11, 0)],
            0.00643,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(12, 0)],
            -0.00953,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(13, 0)],
            -0.02095,
            max_relative = 0.01
        ));
    }

    #[test]
    fn reactions_1() {
        let (elements, nodes) = common::get_structure_fem_matriisit_releases();
        let loads = common::get_fem_matriisi_loads();
        let mut equation_handler = vputilslib::equation_handler::EquationHandler::new();
        let gl_stiff_m = idfem::fem::stiffness::create_joined_stiffness_matrix(&elements, &nodes);
        let gl_eq_loads_m = idfem::fem::equivalent_loads::create_joined_equivalent_loads(
            &elements,
            &nodes,
            &loads,
            &mut equation_handler,
        );
        let displacement = idfem::fem::fem_handler::calculate_displacements(
            &nodes,
            idfem::fem::utils::col_height(&nodes, &elements),
            &gl_stiff_m,
            &gl_eq_loads_m,
        );
        let reactions = idfem::fem::fem_handler::calculate_reactions(
            &gl_stiff_m,
            &displacement,
            &gl_eq_loads_m,
        );
        println!("{}", reactions);
        assert!(relative_eq!(
            reactions[(0, 0)],
            -2.875e4,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            reactions[(1, 0)],
            3.000e4,
            max_relative = 0.01
        ));
        assert!(relative_eq!(reactions[(2, 0)], 3.50e7, max_relative = 0.01));
        assert!((reactions[(3, 0)].round() == 0.0));
        assert!((reactions[(4, 0)].round() == 0.0));
        assert!((reactions[(5, 0)].round() == 0.0));
        assert!(relative_eq!(reactions[(6, 0)], 8.75e3, max_relative = 0.01));
        assert!(relative_eq!(reactions[(7, 0)], 3.00e4, max_relative = 0.01));
        assert!(relative_eq!(reactions[(8, 0)], 4.99e6, max_relative = 0.01));
        assert!((reactions[(9, 0)].round() == 0.0));
        assert!((reactions[(10, 0)].round() == 0.0));
        assert!((reactions[(11, 0)].round() == 0.0));
        assert!((reactions[(12, 0)].round() == 0.0));
        assert!((reactions[(13, 0)].round() == 0.0));
    }

    #[test]
    fn rotated_stiffness_matrix() {
        let end_point: VpPoint = geometry2d::rotate_point(
            &VpPoint::new(0.0, 0.0),
            &VpPoint::new(8000.0, 0.0),
            22.0243128370,
        );

        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, end_point));

        let e: Element = Element::new(
            1,
            1,
            2,
            Profile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 200_000_000.0,
                custom_area: 6000.0,
                ..Profile::default()
            },
            MaterialType::Steel(Steel::new(200.0)),
        );
        let e_glob_stiff_matrix =
            idfem::fem::stiffness::get_element_global_stiffness_matrix(&e, &nodes) / 200.0;
        println!("{}", e_glob_stiff_matrix);
        assert!(relative_eq!(
            e_glob_stiff_matrix[(0, 0)],
            0.6451,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(0, 1)],
            0.2590,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(0, 2)],
            -7.0312,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(0, 3)],
            -0.6451,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(0, 4)],
            -0.2590,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(0, 5)],
            -7.0312,
            max_relative = 0.001
        ));

        assert!(relative_eq!(
            e_glob_stiff_matrix[(1, 0)],
            0.2590,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(1, 1)],
            0.1094,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(1, 2)],
            17.3817,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(1, 3)],
            -0.2590,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(1, 4)],
            -0.1094,
            max_relative = 0.001
        ));

        assert!(relative_eq!(
            e_glob_stiff_matrix[(2, 0)],
            -7.0312,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(2, 2)],
            1e5,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(2, 3)],
            7.0312,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(2, 5)],
            0.5e5,
            max_relative = 0.001
        ));

        assert!(relative_eq!(
            e_glob_stiff_matrix[(5, 0)],
            -7.0312,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(5, 1)],
            17.3817,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(5, 2)],
            0.5e5,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(5, 4)],
            -17.3817,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            e_glob_stiff_matrix[(5, 5)],
            1e5,
            max_relative = 0.001
        ));
    }

    #[test]
    fn joined_stiffness_matrix_fes() {
        let end_point: VpPoint = geometry2d::rotate_point(
            &VpPoint::new(0.0, 0.0),
            &VpPoint::new(8000.0, 0.0),
            22.0243128370,
        );

        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(2, end_point));
        nodes.insert(
            3,
            Node::new_hinged(3, VpPoint::new(nodes[&2].point.x + 8000.0, 3000.0)),
        );

        let e1: Element = Element::new(
            1,
            1,
            2,
            Profile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 200_000_000.0,
                custom_area: 6000.0,
                ..Profile::default()
            },
            MaterialType::Steel(Steel::new(200.0)),
        );
        let e2: Element = Element::new(
            2,
            2,
            3,
            Profile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 200_000_000.0,
                custom_area: 6000.0,
                ..Profile::default()
            },
            // Note the elastic modulus of 200. Comes from the source material
            MaterialType::Steel(Steel::new(200.0)),
        );
        let joined = idfem::fem::stiffness::create_joined_stiffness_matrix(&vec![e1, e2], &nodes);
        for i in 0..9 {
            for j in 0..9 {
                // Note the multiplication of 200. Comes from the source material
                print!("{:>12.04}, ", joined[(i, j)] / 200.0);
            }
            println!();
        }

        // Test the matrix cells that overlap (the same node for both elements)
        // Note the multiplication of 200. Comes from the source material
        assert!(relative_eq!(
            joined[(3, 3)],
            1.3952 * 200.0,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            joined[(4, 3)],
            0.2591 * 200.0,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            joined[(5, 3)],
            7.0312 * 200.0,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            joined[(3, 4)],
            0.2591 * 200.0,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            joined[(4, 4)],
            0.1142 * 200.0,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            joined[(5, 4)],
            1.3683 * 200.0,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            joined[(3, 5)],
            7.0312 * 200.0,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            joined[(4, 5)],
            1.3683 * 200.0,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            joined[(5, 5)],
            2.0e5 * 200.0,
            max_relative = 0.001
        ));
    }

    #[test]
    fn joined_stiffness_matrix_fem_matriisit() {
        // See theory folders xls file (text is in finnish)

        let (elements, nodes) = common::get_structure_fem_matriisit();

        let joined = idfem::fem::stiffness::create_joined_stiffness_matrix(&elements, &nodes);
        for i in 0..12 {
            for j in 0..12 {
                let val = joined[(i, j)];
                if val.abs() < 0.001 {
                    print!("{:>9.0}, ", joined[(i, j)]);
                } else {
                    print!("{:>9.02e}, ", joined[(i, j)]);
                }
            }
            println!();
        }

        // Test the matrix cells that overlap (the same node for both elements)
        assert!(relative_eq!(joined[(3, 3)], 7.00e05, max_relative = 0.01));
        assert!((joined[(4, 3)].round() == 0.0));
        assert!(relative_eq!(joined[(5, 3)], 6.56e05, max_relative = 0.01));
        assert!((joined[(3, 4)].round() == 0.0));
        assert!(relative_eq!(joined[(4, 4)], 5.26e05, max_relative = 0.01));
        assert!(relative_eq!(joined[(5, 4)], 2.33e06, max_relative = 0.01));
        assert!(relative_eq!(joined[(3, 5)], 6.56e05, max_relative = 0.01));
        assert!(relative_eq!(joined[(4, 5)], 2.33e06, max_relative = 0.01));
        assert!(relative_eq!(joined[(5, 5)], 1.11e10, max_relative = 0.01));

        assert!(relative_eq!(joined[(9, 9)], 7.00e05, max_relative = 0.01));
        assert!((joined[(10, 9)].round() == 0.0));
        assert!(relative_eq!(joined[(11, 9)], 6.56e05, max_relative = 0.01));
        assert!((joined[(9, 10)].round() == 0.0));
        assert!(relative_eq!(joined[(10, 10)], 5.26e05, max_relative = 0.01));
        assert!(relative_eq!(
            joined[(11, 10)],
            -2.33e06,
            max_relative = 0.01
        ));
        assert!(relative_eq!(joined[(9, 11)], 6.56e05, max_relative = 0.01));
        assert!(relative_eq!(
            joined[(10, 11)],
            -2.33e06,
            max_relative = 0.01
        ));
        assert!(relative_eq!(joined[(11, 11)], 1.11e10, max_relative = 0.01));
    }

    #[test]
    fn joined_stiffness_matrix_fem_matriisit_releases() {
        // See theory folders xls file (text is in finnish)

        let (elements, nodes) = common::get_structure_fem_matriisit_releases();

        let joined = idfem::fem::stiffness::create_joined_stiffness_matrix(&elements, &nodes);

        for i in 0..14 {
            for j in 0..14 {
                let val = joined[(i, j)];
                if val.abs() < 0.001 {
                    print!("{:>9.0}, ", joined[(i, j)]);
                } else {
                    print!("{:>9.02e}, ", joined[(i, j)]);
                }
            }
            println!();
        }

        // Test the matrix cells that overlap (the same node for both elements)
        // and the cells that are moved because of the release
        assert!(relative_eq!(joined[(3, 3)], 7.00e05, max_relative = 0.01));
        assert!(relative_eq!(joined[(4, 4)], 5.26e05, max_relative = 0.01));
        assert!((joined[(3, 4)].round() == 0.0));

        assert!(relative_eq!(joined[(9, 9)], 7.00e05, max_relative = 0.01));
        assert!(relative_eq!(joined[(10, 10)], 5.26e05, max_relative = 0.01));
        assert!((joined[(9, 10)].round() == 0.0));

        assert!(relative_eq!(joined[(0, 12)], -6.56e05, max_relative = 0.01));
        assert!(relative_eq!(joined[(2, 12)], 8.75e08, max_relative = 0.01));
        assert!(relative_eq!(joined[(3, 12)], 6.56e05, max_relative = 0.01));
        assert!(relative_eq!(joined[(12, 12)], 1.75e09, max_relative = 0.01));

        assert!(relative_eq!(joined[(12, 0)], -6.56e05, max_relative = 0.01));
        assert!(relative_eq!(joined[(12, 2)], 8.75e08, max_relative = 0.01));
        assert!(relative_eq!(joined[(12, 3)], 6.56e05, max_relative = 0.01));

        assert!(relative_eq!(joined[(6, 13)], -6.56e05, max_relative = 0.01));
        assert!(relative_eq!(joined[(8, 13)], 8.75e08, max_relative = 0.01));
        assert!(relative_eq!(joined[(9, 13)], 6.56e05, max_relative = 0.01));
        assert!(relative_eq!(joined[(13, 13)], 1.75e09, max_relative = 0.01));
    }

    #[test]
    fn t_get_unknown_translation_rows() {
        let (elements, nodes) = common::get_structure_fem_matriisit_releases();

        let global_stiff_matrix =
            idfem::fem::stiffness::create_joined_stiffness_matrix(&elements, &nodes);

        let unknown_translation_rows = get_unknown_translation_rows(&nodes, &global_stiff_matrix);
        let stiff_results =
            get_unknown_translation_stiffness_rows(&unknown_translation_rows, &global_stiff_matrix);

        println!("Stiffness matrix:");
        for i in 0..stiff_results.nrows() {
            for j in 0..stiff_results.ncols() {
                let val = stiff_results[(i, j)];
                if val.abs() < 0.001 {
                    print!("{:>9.0}, ", stiff_results[(i, j)]);
                } else {
                    print!("{:>9.02e}, ", stiff_results[(i, j)]);
                }
            }
            println!();
        }
    }
}
