mod common;

#[cfg(test)]
mod readme_test {
    use std::collections::BTreeMap;

    use vefem::loads::LoadGroup;
    use vefem::vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};
    use vefem::{
        loads,
        material::{MaterialData, Steel},
        profile::Profile,
        settings::CalculationSettings,
        structure::{Node, StructureModel},
    };

    #[test]
    fn test_vefem() {
        let n1 = Node::new_hinged(1, VpPoint::new(0.0, 0.0));
        let n2 = Node::new_hinged(2, VpPoint::new(4000.0, 0.0));
        let nodes = BTreeMap::from([(n1.number, n1), (n2.number, n2)]);
        let el = vefem::structure::Element::new(
            1, // Element number
            1, // The node number at the start of the element
            2, // The node number at the end of the element
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::new(210e3)),
        );
        let elements = vec![el];
        let line_load = loads::Load::new_line_load(
            "LineLoad".to_string(),
            "1".to_string(),  // Element number(s)
            "0".to_string(),  // The offset of the loads starts from the start of the element
            "L".to_string(),  // The offset of the loads ends from the start of the element
            "10".to_string(), // in N/mm
            -90.0,
            LoadGroup::PERMANENT,
        ); // 0.0 points towards positive X-axis and goes counterclockwise
        let loads = vec![line_load];
        let mut eq_handler = EquationHandler::new();
        let calc_settings = CalculationSettings::default();
        let calc_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings,
            load_combinations: vec![],
        };
        let results = vefem::fem::fem_handler::calculate(&calc_model, &mut eq_handler);
        // The default settings divide the internal force calculation points into 100 intervals.
        // Assert that the value at the middle of the element is ql^2/8
        assert_eq!(
            results[0].internal_force_results[&1].moment_forces[50].value_y,
            10.0 * 4000f64.powi(2) / 8.0
        );
    }
}

#[cfg(test)]
mod fem_tests {
    use crate::common;
    use approx::relative_eq;
    use std::collections::BTreeMap;
    use std::time::SystemTime;
    use vefem::loads::load_combination::{CalcLoadCombination, LoadCombinationType};
    use vefem::loads::{self, Load, LoadCombination, LoadGroup};
    use vefem::material::{MaterialData, Steel};
    use vefem::profile::{CustomProfile, Profile};
    use vefem::settings::CalculationSettings;
    use vefem::structure::Node;
    use vefem::structure::{Element, StructureModel};
    use vputilslib::equation_handler::EquationHandler;
    use vputilslib::geometry2d;
    use vputilslib::geometry2d::VpPoint;

    #[test]
    fn displacement_1() {
        let (elements, nodes) = common::get_structure_fem_matriisit();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
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
        let calc_model = common::get_calc_model(&elements, &nodes);
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
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
    fn displacement_3() {
        let (elements, mut nodes) = common::get_structure_fem_matriisit_releases();
        nodes.get_mut(&1).unwrap().support.r_spring = 1e9;
        nodes.get_mut(&1).unwrap().support.ry = false;
        nodes.get_mut(&3).unwrap().support.r_spring = 1e9;
        nodes.get_mut(&3).unwrap().support.ry = false;
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
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
            -0.02649,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(3, 0)],
            125.72384,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(4, 0)],
            -0.05714,
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
            -0.01351,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(9, 0)],
            125.70473,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(10, 0)],
            -0.05714,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(11, 0)],
            0.00643,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(12, 0)],
            -0.02628,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(13, 0)],
            -0.04419,
            max_relative = 0.01
        ));
    }

    #[test]
    fn displacement_support_rotated_1() {
        let (elements, nodes) = common::get_structure_for_rotated_support_1();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let loads = common::get_loads_for_rotated_support_1();
        let calc_settings = CalculationSettings::default();
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
        );
        println!("Displacements:");
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
            -0.0381,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(4, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(5, 0)],
            0.0000,
            max_relative = 0.01
        ));
    }

    #[test]
    fn displacement_support_rotated_2() {
        let (elements, nodes) = common::get_structure_for_rotated_support_2();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let loads = common::get_loads_for_rotated_support_2();
        let calc_settings = CalculationSettings::default();
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
        );
        println!("Displacements:");
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
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(4, 0)],
            -182.8571,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(5, 0)],
            -0.06095,
            max_relative = 0.01
        ));
    }

    #[test]
    fn displacement_support_rotated_3() {
        let (elements, mut nodes) = common::get_structure_fem_matriisit();
        nodes.get_mut(&2).unwrap().support.rotation = 90.0;
        nodes.get_mut(&2).unwrap().support.tx = true;
        nodes.get_mut(&4).unwrap().support.rotation = 90.0;
        nodes.get_mut(&4).unwrap().support.tz = true;
        let calc_model = common::get_calc_model(&elements, &nodes);
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
        );
        println!("Displacements:");
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
            -0.006537,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(3, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(4, 0)],
            -0.0367,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(5, 0)],
            -0.0021918,
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
            0.002396,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(9, 0)],
            -0.0542,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(10, 0)],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[(11, 0)],
            0.0028276,
            max_relative = 0.01
        ));
    }

    #[test]
    fn displacement_support_rotated_4() {
        let (elements, mut nodes) = common::get_structure_fem_matriisit();
        nodes.get_mut(&2).unwrap().support.rotation = 45.0;
        nodes.get_mut(&2).unwrap().support.tx = true;
        nodes.get_mut(&4).unwrap().support.rotation = 45.0;
        nodes.get_mut(&4).unwrap().support.tz = true;
        let loads = common::get_fem_matriisi_loads();

        let structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let calc_results = vefem::fem::fem_handler::calculate(&structure_model, &EquationHandler::new());
        let displacement = &calc_results[0].node_results.displacements;
        let global_displacement = &calc_results[0].node_results.global_displacements;
        println!("Local displacements:");
        println!("{:?}", displacement);
        println!("Global displacements:");
        println!("{:?}", global_displacement);
        assert!(relative_eq!(
            displacement[0],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[1],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[2],
            -0.0065444,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[3],
            0.0000,
            max_relative = 0.01
        ));
        // LOCAL DISPLACEMENTS
        assert!(relative_eq!(
            displacement[4],
            -0.0571,
            max_relative = 0.01
        ));
        //
        // GLOBAL DISPLACEMENTS
        assert!(relative_eq!(
            global_displacement[3],
            0.04034,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            global_displacement[4],
            -0.04034,
            max_relative = 0.001
        ));
        //
        assert!(relative_eq!(
            displacement[5],
            -0.00217946,
            max_relative = 0.01
        ));

        assert!(relative_eq!(
            displacement[6],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[7],
            0.0000,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            displacement[8],
            0.0023933,
            max_relative = 0.01
        ));
        // GLOBAL DISPLACEMENTS
        assert!(relative_eq!(
            displacement[9],
            -0.0158,
            max_relative = 0.01
        ));
        //
        assert!(relative_eq!(
            displacement[10],
            0.0000,
            max_relative = 0.01
        ));
        // GLOBAL DISPLACEMENTS
        assert!(relative_eq!(
            global_displacement[9],
            -0.01116,
            max_relative = 0.001
        ));
        assert!(relative_eq!(
            global_displacement[10],
            -0.01116,
            max_relative = 0.001
        ));
        //
        assert!(relative_eq!(
            displacement[11],
            0.0028408,
            max_relative = 0.01
        ));
    }

    #[test]
    fn reactions_1() {
        let (elements, nodes) = common::get_structure_fem_matriisit();
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
        );
        let reactions = vefem::fem::fem_handler::calculate_reactions(
            &calc_matrices.stiffness,
            &displacement,
            &calc_matrices.equivalent_loads,
        );
        println!("{}", reactions);
        assert!(relative_eq!(
            reactions[(0, 0)],
            -2.0427e4,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            reactions[(1, 0)],
            2.3333e4,
            max_relative = 0.01
        ));
        assert_eq!(reactions[(2, 0)].round(), 0.0);
        assert_eq!(reactions[(3, 0)].round(), 0.0);
        assert_eq!(reactions[(4, 0)].round(), 0.0);
        assert_eq!(reactions[(5, 0)].round(), 0.0);
        assert!(relative_eq!(reactions[(6, 0)], 4.27e2, max_relative = 0.01));
        assert!(relative_eq!(reactions[(7, 0)], 3.6666e4, max_relative = 0.01));
        assert_eq!(reactions[(8, 0)].round(), 0.0);
        assert_eq!(reactions[(9, 0)].round(), 0.0);
        assert_eq!(reactions[(10, 0)].round(), 0.0);
        assert_eq!(reactions[(11, 0)].round(), 0.0);
    }

    #[test]
    fn reactions_2() {
        let (elements, nodes) = common::get_structure_fem_matriisit_releases();
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
        );
        let reactions = vefem::fem::fem_handler::calculate_reactions(
            &calc_matrices.stiffness,
            &displacement,
            &calc_matrices.equivalent_loads,
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
        assert_eq!(reactions[(3, 0)].round(), 0.0);
        assert_eq!(reactions[(4, 0)].round(), 0.0);
        assert_eq!(reactions[(5, 0)].round(), 0.0);
        assert!(relative_eq!(reactions[(6, 0)], 8.75e3, max_relative = 0.01));
        assert!(relative_eq!(reactions[(7, 0)], 3.00e4, max_relative = 0.01));
        assert!(relative_eq!(reactions[(8, 0)], 4.99e6, max_relative = 0.01));
        assert_eq!(reactions[(9, 0)].round(), 0.0);
        assert_eq!(reactions[(10, 0)].round(), 0.0);
        assert_eq!(reactions[(11, 0)].round(), 0.0);
        assert_eq!(reactions[(12, 0)].round(), 0.0);
        assert_eq!(reactions[(13, 0)].round(), 0.0);
    }

    #[test]
    fn reactions_3() {
        let (elements, mut nodes) = common::get_structure_fem_matriisit_releases();
        nodes.get_mut(&1).unwrap().support.r_spring = 1e9;
        nodes.get_mut(&1).unwrap().support.ry = false;
        nodes.get_mut(&3).unwrap().support.r_spring = 1e9;
        nodes.get_mut(&3).unwrap().support.ry = false;
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
        );
        let reactions = vefem::fem::fem_handler::calculate_reactions(
            &calc_matrices.stiffness,
            &displacement,
            &calc_matrices.equivalent_loads,
        );
        println!("{}", reactions);
        assert!(relative_eq!(
            reactions[(0, 0)],
            -2.6622e4,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            reactions[(1, 0)],
            3.000e4,
            max_relative = 0.01
        ));
        assert!(relative_eq!(
            reactions[(2, 0)],
            2.64878e7,
            max_relative = 0.01
        ));
        assert_eq!(reactions[(3, 0)].round(), 0.0);
        assert_eq!(reactions[(4, 0)].round(), 0.0);
        assert_eq!(reactions[(5, 0)].round(), 0.0);
        assert!(relative_eq!(
            reactions[(6, 0)],
            6.622e3,
            max_relative = 0.01
        ));
        assert!(relative_eq!(reactions[(7, 0)], 3.00e4, max_relative = 0.01));
        assert!(relative_eq!(
            reactions[(8, 0)],
            1.3512158e7,
            max_relative = 0.01
        ));
        assert_eq!(reactions[(9, 0)].round(), 0.0);
        assert_eq!(reactions[(10, 0)].round(), 0.0);
        assert_eq!(reactions[(11, 0)].round(), 0.0);
        assert_eq!(reactions[(12, 0)].round(), 0.0);
        assert_eq!(reactions[(13, 0)].round(), 0.0);
    }

    #[test]
    fn reactions_rotated_1() {
        let (elements, mut nodes) = common::get_structure_fem_matriisit();
        nodes.get_mut(&2).unwrap().support.rotation = 90.0;
        nodes.get_mut(&2).unwrap().support.tx = true;
        nodes.get_mut(&4).unwrap().support.rotation = 90.0;
        nodes.get_mut(&4).unwrap().support.tz = true;
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
        );
        let _reactions = vefem::fem::fem_handler::calculate_reactions(
            &calc_matrices.stiffness,
            &displacement,
            &calc_matrices.equivalent_loads,
        );
        let structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let calc_results = vefem::fem::fem_handler::calculate(&structure_model, &EquationHandler::new());
        println!("Displacements:");
        println!("{:?}", calc_results[0].node_results.displacements);
        println!("Displacements (GLOBAL):");
        println!("{:?}", calc_results[0].node_results.global_displacements);
        println!("Support reactions:");
        println!("{:?}", calc_results[0].node_results.support_reactions);
        let reactions = &calc_results[0].node_results.support_reactions;
        assert!(relative_eq!(reactions[0], -1.42838e4, max_relative = 0.01 ));
        assert_eq!(reactions[1].round(), 0.0);
        assert_eq!(reactions[2].round(), 0.0);
        assert!(relative_eq!(reactions[3], 3.1525588e4, max_relative = 0.01 ));
        assert_eq!(reactions[4].round(), 0.0);
        assert_eq!(reactions[5].round(), 0.0);
        assert!(relative_eq!(reactions[6], 6.57220e3, max_relative = 0.01));
        assert!(relative_eq!(reactions[7], 2.847441e4, max_relative = 0.01));
        assert_eq!(reactions[8].round(), 0.0);
        assert_eq!(reactions[9].round(), 0.0);
        assert!(relative_eq!(reactions[10], 1.228838e4, max_relative = 0.01));
        assert_eq!(reactions[11].round(), 0.0);
    }

    #[test]
    fn reactions_rotated_2() {
        let (elements, mut nodes) = common::get_structure_fem_matriisit();
        nodes.get_mut(&2).unwrap().support.rotation = 45.0;
        nodes.get_mut(&2).unwrap().support.tx = true;
        nodes.get_mut(&4).unwrap().support.rotation = 45.0;
        nodes.get_mut(&4).unwrap().support.tz = true;
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            &CalcLoadCombination::default(),
            &EquationHandler::new(),
        );
        let mut calc_matrices = vefem::fem::fem_handler::create_global_calculation_matrix(&calc_model, &calc_settings, &calc_loads);
        let displacement = vefem::fem::fem_handler::calculate_displacements(
            &nodes,
            vefem::fem::utils::col_height(&nodes, &elements),
            &mut calc_matrices.stiffness,
            &mut calc_matrices.equivalent_loads,
        );
        let _reactions = vefem::fem::fem_handler::calculate_reactions(
            &calc_matrices.stiffness,
            &displacement,
            &calc_matrices.equivalent_loads,
        );
        let structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let calc_results = vefem::fem::fem_handler::calculate(&structure_model, &EquationHandler::new());
        println!("Displacements:");
        println!("{:?}", calc_results[0].node_results.displacements);
        println!("Displacements (GLOBAL):");
        println!("{:?}", calc_results[0].node_results.global_displacements);
        println!("Support reactions:");
        println!("{:?}", calc_results[0].node_results.support_reactions);
        let reactions = &calc_results[0].node_results.support_reactions;
        assert!(relative_eq!(reactions[0], -1.4288e4, max_relative = 0.01 ));
        assert!(relative_eq!(reactions[1], 2.11802e4, max_relative = 0.01 ));
        assert_eq!(reactions[2].round(), 0.0);
        assert!(relative_eq!(reactions[3], 1.4623e4, max_relative = 0.01 ));
        assert_eq!(reactions[4].round(), 0.0);
        assert_eq!(reactions[5].round(), 0.0);
        assert!(relative_eq!(reactions[6], 6.5688e3, max_relative = 0.01));
        assert!(relative_eq!(reactions[7], 5.8588e3, max_relative = 0.01));
        assert_eq!(reactions[8].round(), 0.0);
        assert_eq!(reactions[9].round(), 0.0);
        assert!(relative_eq!(reactions[10], 3.19906e4, max_relative = 0.01));
        assert_eq!(reactions[11].round(), 0.0);
    }

    #[test]
    fn rotated_stiffness_matrix() {
        let end_point: VpPoint = geometry2d::rotate_point(
            &VpPoint::new(0.0, 0.0),
            &VpPoint::new(8000.0, 0.0),
            22.0243128370,
        );
        let calc_settings = CalculationSettings::default();

        let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, end_point));

        let e: Element = Element::new(
            1,
            1,
            2,
            Profile::CustomProfile(CustomProfile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 200_000_000.0,
                custom_area: 6000.0,
                ..Default::default()
            }),
            MaterialData::Steel(Steel::new(200.0)),
        );
        let elements = vec![e];
        let calc_model = common::get_calc_model(&elements, &nodes);
        let e_glob_stiff_matrix =
            vefem::fem::stiffness::get_element_global_stiffness_matrix(&calc_model.get_all_calc_elements()[0], &calc_settings) / 200.0;
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
    fn moments_1() {
        let (elements, nodes) = common::get_structure_fem_matriisit();
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let struct_model = StructureModel {
            elements,
            nodes,
            loads,
            calc_settings,
            load_combinations: vec![],
        };
        let results = vefem::fem::fem_handler::calculate(&struct_model, &EquationHandler::new());

        println!();
        println!("Moment (el: 1) at L: {} kNm", 
            results[0].internal_force_results[&1].get_force_at(vefem::results::ForceType::Moment, 4000.0).unwrap().value_y * 1e-6
        );
        println!("Moment (el: 2) at 0: {} kNm", 
            results[0].internal_force_results[&2].get_force_at(vefem::results::ForceType::Moment, 0.0).unwrap().value_y * 1e-6
        );

        println!("Moment (el: 2) at L: {} kNm", 
            results[0].internal_force_results[&2].get_force_at(vefem::results::ForceType::Moment, 6000.0).unwrap().value_y * 1e-6
        );
        println!("Moment (el: 3) at L: {} kNm", 
            results[0].internal_force_results[&3].get_force_at(vefem::results::ForceType::Moment, 4000.0).unwrap().value_y * 1e-6
        );
    }

    #[test]
    fn moments_2() {
        let (elements, nodes) = common::get_structure_three_horizontal_elements();
        let load = Load::new_line_load(
            "test".to_string(),
            "-1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
            LoadGroup::PERMANENT
        );
        let loads = vec![load];
        let calc_settings = CalculationSettings::default();
        let struct_model = StructureModel {
            elements,
            nodes,
            loads,
            calc_settings,
            load_combinations: vec![],
        };
        let results = vefem::fem::fem_handler::calculate(&struct_model, &EquationHandler::new());

        println!();
        println!("Moment (el: 1) at 1,6 m: {} kNm",
                 results[0].internal_force_results[&1].get_force_at(vefem::results::ForceType::Moment, 1600.0).unwrap().value_y * 1e-6
        );
        println!("Moment (el: 2) at 0: {} kNm",
                 results[0].internal_force_results[&2].get_force_at(vefem::results::ForceType::Moment, 4000.0).unwrap().value_y * 1e-6
        );

        println!("Moment (el: 2) at 2 m: {} kNm",
                 results[0].internal_force_results[&2].get_force_at(vefem::results::ForceType::Moment, 2000.0).unwrap().value_y * 1e-6
        );
    }

    #[test]
    fn moments_3() {
        let mut nodes: BTreeMap<i32, Node> = BTreeMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0)));

        let e1: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::new(210e3)),
        );

        let elements = vec![e1];

        let load = Load::new_line_load(
            "test".to_string(),
            "-1".to_string(),
            "0".to_string(),
            "1000".to_string(),
            "10".to_string(),
            -90.0,
            LoadGroup::PERMANENT
        );
        let loads = vec![load];
        let calc_settings = CalculationSettings::default();
        let struct_model = StructureModel {
            elements,
            nodes, loads,
            calc_settings,
            load_combinations: vec![],
        };
        let results = vefem::fem::fem_handler::calculate(&struct_model, &EquationHandler::new());
        println!();
        println!("Support reaction start: {}", results[0].node_results.support_reactions[1]);
        println!("Support reaction end: {}", results[0].node_results.support_reactions[4]);
        println!("Moment (el: 1) at 1,0 m: {} kNm",
                 results[0].internal_force_results[&1].get_force_at(vefem::results::ForceType::Moment, 1000.0).unwrap().value_y * 1e-6
        );
        println!("Moment (el: 1) at 2,0 m: {} kNm",
                 results[0].internal_force_results[&1].get_force_at(vefem::results::ForceType::Moment, 2000.0).unwrap().value_y * 1e-6
        );
        println!("Moment (el: 1) at 3,0 m: {} kNm",
                 results[0].internal_force_results[&1].get_force_at(vefem::results::ForceType::Moment, 3000.0).unwrap().value_y * 1e-6
        );
        println!("Moment (el: 1) at 4,0 m: {} kNm",
                 results[0].internal_force_results[&1].get_force_at(vefem::results::ForceType::Moment, 4000.0).unwrap().value_y * 1e-6
        );
        assert!((results[0].internal_force_results[&1].get_force_at(vefem::results::ForceType::Moment, 1000.0).unwrap().value_y - 3.75e6).abs() < 0.1);
    }

    macro_rules! internal_force_test {
        ($results:expr, $force_type:expr, $el_num:expr, $location:expr, $expected:expr) => {
            let force = $results[0].internal_force_results[&$el_num].get_force_at($force_type, $location).unwrap().value_y;
            println!("{:?} force (el: {}) at L: {}", $force_type, $el_num, $location);
            println!("{force}");
            assert!(relative_eq!(
                force,
                $expected,
                max_relative = 0.01
            ));
        };
    }

    #[test]
    fn internal_forces_rotated_supports_1() {
        let (elements, mut nodes) = common::get_structure_fem_matriisit();
        nodes.get_mut(&2).unwrap().support.rotation = 45.0;
        nodes.get_mut(&2).unwrap().support.tx = true;
        nodes.get_mut(&4).unwrap().support.rotation = 45.0;
        nodes.get_mut(&4).unwrap().support.tz = true;
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let struct_model = StructureModel {
            elements,
            nodes,
            loads,
            calc_settings,
            load_combinations: vec![],
        };
        let results = vefem::fem::fem_handler::calculate(&struct_model, &EquationHandler::new());

        internal_force_test!(results, vefem::results::ForceType::Axial, 1, 0.0, -2.12e4);
        internal_force_test!(results, vefem::results::ForceType::Axial, 2, 0.0, -3.61e4);
        internal_force_test!(results, vefem::results::ForceType::Axial, 3, 0.0, -5.86e3);

        internal_force_test!(results, vefem::results::ForceType::Shear, 1, 0.0, 1.43e4);
        internal_force_test!(results, vefem::results::ForceType::Shear, 1, 4000.0, -2.57e4);
        internal_force_test!(results, vefem::results::ForceType::Shear, 2, 0.0, 3.15e4);
        internal_force_test!(results, vefem::results::ForceType::Shear, 2, 6000.0, -2.85e4);
        internal_force_test!(results, vefem::results::ForceType::Shear, 3, 0.0, -6.57e3);
        internal_force_test!(results, vefem::results::ForceType::Shear, 3, 4000.0, 1.34e4);

        internal_force_test!(results, vefem::results::ForceType::Moment, 1, 4000.0, -2.285e7);
        internal_force_test!(results, vefem::results::ForceType::Moment, 2, 0.0, -2.285e7);
        internal_force_test!(results, vefem::results::ForceType::Moment, 2, 6000.0, -1.372e7);
        internal_force_test!(results, vefem::results::ForceType::Moment, 3, 4000.0, 1.372e7);

        internal_force_test!(results, vefem::results::ForceType::Moment, 1, 1500.0, 1.018e7);
        internal_force_test!(results, vefem::results::ForceType::Moment, 1, 2000.0, 8.576e6);
        internal_force_test!(results, vefem::results::ForceType::Moment, 2, 3000.0, 2.671e7);
        internal_force_test!(results, vefem::results::ForceType::Moment, 3, 2000.0, -3.138e6);
    }

    #[ignore]
    #[test]
    fn moments_non_threaded() {
        let (elements, nodes) = common::get_structure_fem_matriisit();
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings { calc_threaded: false, ..Default::default() };

        let timer = SystemTime::now();
        let count = 20;
        let mut load_combinations = vec![];

        println!("Initializing the load combinations...");
        for i in 1..count {
            load_combinations.push(LoadCombination::new(i, format!("{i}"), LoadCombinationType::None));
        }

        let struct_model = StructureModel {
            elements,
            nodes,
            loads,
            calc_settings,
            load_combinations,
        };

        println!("Starting to calculate...");
        println!("Time: {:?}", timer.elapsed().unwrap());
        vefem::fem::fem_handler::calculate(&struct_model, &EquationHandler::new());
        println!("Calculations done.");

        println!("Time: {:?}", timer.elapsed().unwrap());
    }

    // Test function to test the speed compared to non-threaded calculations
    #[ignore]
    #[test]
    fn moments_threaded() {
        let (elements, nodes) = common::get_structure_fem_matriisit();
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings { calc_threaded: true, ..Default::default() };

        let timer = SystemTime::now();
        let count = 500;
        let mut load_combinations = vec![];

        println!("Initializing the load combinations...");
        for i in 1..count {
            load_combinations.push(LoadCombination::new(i, format!("{i}"), LoadCombinationType::None));
        }

        let struct_model = StructureModel {
            elements,
            nodes,
            loads,
            calc_settings,
            load_combinations,
        };

        println!("Starting to calculate...");
        println!("Time: {:?}", timer.elapsed().unwrap());
        vefem::fem::fem_handler::calculate(&struct_model, &EquationHandler::new());
        println!("Calculations done.");

        println!("Time: {:?}", timer.elapsed().unwrap());
    }
    
    #[test]
    fn strain_load() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_free(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_fixed(2, VpPoint::new(2828.57, 2828.57))),
        ]);
        let elements = vec![el];
        let p_load = Load::new_strain_load(
            "StrainLoad".to_string(),
            "-1".to_string(),
            "-10".to_string(),
            LoadGroup::PERMANENT,
        );
        let loads = vec![p_load];
        let struct_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results = vefem::fem::fem_handler::calculate(&struct_model, &EquationHandler::new());
        println!();
        println!("Support reaction start: {}", results[0].node_results.support_reactions[0]);
        println!("Support reaction start: {}", results[0].node_results.support_reactions[1]);
    }
}
