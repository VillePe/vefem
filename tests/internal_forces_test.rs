mod common;

#[cfg(test)]
mod internal_forces_tests {
    use crate::common;
    use std::collections::{BTreeMap, HashMap};

    use approx::relative_eq;
    use vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};

    use vefem::{
        fem::CalcModel,
        loads::{Load, LoadCombination},
        material::{MaterialData, Steel},
        profile::Profile,
        results::ForceType,
        settings::CalculationSettings,
        structure::{Element, Node, StructureModel},
    };

    #[test]
    fn t_calculate_moment_at_pl() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let p_load = Load::new_point_load(
            "Pointload".to_string(),
            "1".to_string(),
            "L/2".to_string(),
            "10".to_string(),
            -90.0,
        );
        let loads = vec![p_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 1000.0)
            .unwrap()
            .value_y;
        println!("Moment(1000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 5e6, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 10e6, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 3000.0)
            .unwrap()
            .value_y;
        println!("Moment(3000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 5e6, epsilon = 1.0), true);

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000<-45): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 10e6 / 2f64.sqrt(), epsilon = 1.0), true);

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000<45): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, -10e6 / 2f64.sqrt(), epsilon = 1.0), true);

        structure_model.loads[0].rotation = 0.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(horizontal): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 0.0, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_moment_at_rl() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let r_load = Load::new_rotational_load(
            "RotationalLoad".to_string(),
            "1".to_string(),
            "L/2".to_string(),
            "10".to_string(),
        );
        let loads = vec![r_load];
        let structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 1000.0)
            .unwrap()
            .value_y;
        println!("Moment(1000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 2.5e6, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 1960.0)
            .unwrap()
            .value_y;
        println!("Moment(1960): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 2500.0 * 1960.0, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, -5e6, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 3000.0)
            .unwrap()
            .value_y;
        println!("Moment(3000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, -2.5e6, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_moment_at_ll() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_line_load(
            "Lineload".to_string(),
            "1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 1000.0)
            .unwrap()
            .value_y;
        println!("Moment(1000): {} kNm", mom / 1e6);
        assert_eq!(
            relative_eq!(
                mom,
                (20000.0 * 1000.0 - (10.0 * 1000.0 * 1000.0 / 2.0)),
                epsilon = 1.0
            ),
            true
        );
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000): {} kNm", mom / 1e6);
        assert_eq!(
            relative_eq!(mom, 10.0 * 4000f64.powi(2) / 8.0, epsilon = 1.0),
            true
        );
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 3000.0)
            .unwrap()
            .value_y;
        println!("Moment(3000): {} kNm", mom / 1e6);
        assert_eq!(
            relative_eq!(
                mom,
                (20000.0 * 1000.0 - (10.0 * 1000.0 * 1000.0 / 2.0)),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000<-45): {} kNm", mom / 1e6);
        assert_eq!(
            relative_eq!(
                mom,
                10.0 * 4000f64.powi(2) / 8.0 / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000<45): {} kNm", mom / 1e6);
        assert_eq!(
            relative_eq!(
                mom,
                -10.0 * 4000f64.powi(2) / 8.0 / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 0.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(horizontal): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 0.0, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_moment_at_tl_ltr_full() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_triangular_load(
            "TriangularLoad".to_string(),
            "1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 1000.0)
            .unwrap()
            .value_y;
        println!("Moment(1000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 8.75e6, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 10.00e6, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 3000.0)
            .unwrap()
            .value_y;
        println!("Moment(3000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 6.25e6, epsilon = 1.0), true);

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000<-45): {} kNm", mom / 1e6);
        assert_eq!(
            relative_eq!(mom, 10.00e6 / 2f64.sqrt(), epsilon = 1.0),
            true
        );

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000<45): {} kNm", mom / 1e6);
        assert_eq!(
            relative_eq!(mom, -10.00e6 / 2f64.sqrt(), epsilon = 1.0),
            true
        );

        structure_model.loads[0].rotation = 0.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(horizontal)): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 0.0, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_moment_at_tl_rtl_full() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_triangular_load(
            "TriangularLoad".to_string(),
            "1".to_string(),
            "L".to_string(),
            "0".to_string(),
            "10".to_string(),
            -90.0,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 1000.0)
            .unwrap()
            .value_y;
        println!("Moment(1000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 6.25e6, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 10.00e6, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 3000.0)
            .unwrap()
            .value_y;
        println!("Moment(3000): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 8.75e6, epsilon = 1.0), true);

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000<-45): {} kNm", mom / 1e6);
        assert_eq!(
            relative_eq!(mom, 10.00e6 / 2f64.sqrt(), epsilon = 1.0),
            true
        );

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000<45): {} kNm", mom / 1e6);
        assert_eq!(
            relative_eq!(mom, -10.00e6 / 2f64.sqrt(), epsilon = 1.0),
            true
        );

        structure_model.loads[0].rotation = 0.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(horizontal)): {} kNm", mom / 1e6);
        assert_eq!(relative_eq!(mom, 0.0, epsilon = 1.0), true);
    }

    #[test]
    fn t_calc_moment_with_middle_supports_ll() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
            (3, Node::new_hinged(3, VpPoint::new(1000.0, 0.0))),
            (4, Node::new_hinged(4, VpPoint::new(2000.0, 0.0))),
            (5, Node::new_hinged(5, VpPoint::new(3000.0, 0.0))),
        ]);
        let elements = vec![el];
        let load = Load::new_line_load(
            "LineLoad".to_string(),
            "1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10,20".to_string(),
            -90.0,
        );
        let loads = vec![load];
        let structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        for i in (0..4000).step_by(200) {
            let mom = results.internal_force_results[&1]
                .get_force_at(ForceType::Moment, i as f64)
                .unwrap()
                .value_y;
            println!("Moment({}): {} kNm", i, mom / 1e6);
        }
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 1000.0)
            .unwrap()
            .value_y;
        println!("Moment(1000): {} kNm", mom / 1e6);
        // assert_eq!(relative_eq!(mom, 21.25e6, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 2000.0)
            .unwrap()
            .value_y;
        println!("Moment(2000): {} kNm", mom / 1e6);
        // assert_eq!(relative_eq!(mom, 30.00e6, epsilon = 1.0), true);
        let mom = results.internal_force_results[&1]
            .get_force_at(ForceType::Moment, 3000.0)
            .unwrap()
            .value_y;
        println!("Moment(3000): {} kNm", mom / 1e6);
        // assert_eq!(relative_eq!(mom, 23.75e6, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_shear_at_pl() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let p_load = Load::new_point_load(
            "Pointload".to_string(),
            "1".to_string(),
            "L/2".to_string(),
            "10".to_string(),
            -90.0,
        );
        let loads = vec![p_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 1000.0)
            .unwrap()
            .value_y;
        println!("Shear(1000): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 5e3, epsilon = 1.0), true);
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, -5e3, epsilon = 1.0), true);
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 3000.0)
            .unwrap()
            .value_y;
        println!("Shear(3000): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, -5e3, epsilon = 1.0), true);

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000<-45): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, -5e3 / 2f64.sqrt(), epsilon = 1.0), true);

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000<45): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 5e3 / 2f64.sqrt(), epsilon = 1.0), true);

        structure_model.loads[0].rotation = 0.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(horizontal): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 0.0, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_shear_at_rl() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let r_load = Load::new_rotational_load(
            "RotationalLoad".to_string(),
            "1".to_string(),
            "L/2".to_string(),
            "10".to_string(),
        );
        let loads = vec![r_load];
        let structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 1000.0)
            .unwrap()
            .value_y;

        println!("Shear(1000): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 2.5e3, epsilon = 1.0), true);
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 1999.0)
            .unwrap()
            .value_y;
        println!("Shear(1999): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 2.5e3, epsilon = 1.0), true);
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 2.5e3, epsilon = 1.0), true);
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 3000.0)
            .unwrap()
            .value_y;
        println!("Shear(3000): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 2.5e3, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_shear_at_ll() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_line_load(
            "Lineload".to_string(),
            "1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 1000.0)
            .unwrap()
            .value_y;

        println!("Shear(1000): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(shear, (10.0 * 4.0 / 2.0 - 10.0 * 1.0) * 1e3, epsilon = 1.0),
            true
        );
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 0.0, epsilon = 1.0), true);
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 3000.0)
            .unwrap()
            .value_y;
        println!("Shear(3000): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(shear, (10.0 * 4.0 / 2.0 - 10.0 * 3.0) * 1e3, epsilon = 1.0),
            true
        );

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 1000.0)
            .unwrap()
            .value_y;
        println!("Shear(1000<-45): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                (10.0 * 4.0 / 2.0 - 10.0 * 1.0) * 1e3 / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 1000.0)
            .unwrap()
            .value_y;
        println!("Shear(1000<45): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                -(10.0 * 4.0 / 2.0 - 10.0 * 1.0) * 1e3 / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 0.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 1000.0)
            .unwrap()
            .value_y;
        println!("Shear(horizontal): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 0.0, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_shear_at_tl_ltr_full() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_triangular_load(
            "TriangularLoad".to_string(),
            "1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 1000.0)
            .unwrap()
            .value_y;

        println!("Shear(1000): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                (10.0 * 4.0 / 2.0 * 2.0 / 3.0
                    - 10.0 * (4.0 - 1.0) / 4.0 * 1.0
                    - (10.0 - 10.0 * (4.0 - 1.0) / 4.0) * 1.0 / 2.0)
                    * 1e3,
                epsilon = 1.0
            ),
            true
        );
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                (10.0 * 4.0 / 2.0 * 2.0 / 3.0
                    - 10.0 * (4.0 - 2.0) / 4.0 * 2.0
                    - (10.0 - 10.0 * (4.0 - 2.0) / 4.0) * 2.0 / 2.0)
                    * 1e3,
                epsilon = 1.0
            ),
            true
        );
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 3000.0)
            .unwrap()
            .value_y;
        println!("Shear(3000): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                (10.0 * 4.0 / 2.0 * 2.0 / 3.0
                    - 10.0 * (4.0 - 3.0) / 4.0 * 3.0
                    - (10.0 - 10.0 * (4.0 - 3.0) / 4.0) * 3.0 / 2.0)
                    * 1e3,
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000<-45): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                (10.0 * 4.0 / 2.0 * 2.0 / 3.0
                    - 10.0 * (4.0 - 2.0) / 4.0 * 2.0
                    - (10.0 - 10.0 * (4.0 - 2.0) / 4.0) * 2.0 / 2.0)
                    * 1e3
                    / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000<45): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                -(10.0 * 4.0 / 2.0 * 2.0 / 3.0
                    - 10.0 * (4.0 - 2.0) / 4.0 * 2.0
                    - (10.0 - 10.0 * (4.0 - 2.0) / 4.0) * 2.0 / 2.0)
                    * 1e3
                    / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 0.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(horizontal)): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 0.0, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_shear_at_tl_rtl_full() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_triangular_load(
            "TriangularLoad".to_string(),
            "1".to_string(),
            "L".to_string(),
            "0".to_string(),
            "10".to_string(),
            -90.0,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];

        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 1000.0)
            .unwrap()
            .value_y;
        println!("Shear(1000): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                (10.0 * 4.0 / 2.0 * 1.0 / 3.0 - 10.0 * (1.0) / 4.0 * 1.0 / 2.0) * 1e3,
                epsilon = 1.0
            ),
            true
        );
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                (10.0 * 4.0 / 2.0 * 1.0 / 3.0 - 10.0 * (2.0) / 4.0 * 2.0 / 2.0) * 1e3,
                epsilon = 1.0
            ),
            true
        );
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 3000.0)
            .unwrap()
            .value_y;
        println!("Shear(3000): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                (10.0 * 4.0 / 2.0 * 1.0 / 3.0 - 10.0 * (3.0) / 4.0 * 3.0 / 2.0) * 1e3,
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000<-45): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                (10.0 * 4.0 / 2.0 * 1.0 / 3.0 - 10.0 * (2.0) / 4.0 * 2.0 / 2.0) * 1e3 / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(2000<45): {} kN", shear / 1e3);
        assert_eq!(
            relative_eq!(
                shear,
                -(10.0 * 4.0 / 2.0 * 1.0 / 3.0 - 10.0 * (2.0) / 4.0 * 2.0 / 2.0) * 1e3
                    / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 0.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let shear = results.internal_force_results[&1]
            .get_force_at(ForceType::Shear, 2000.0)
            .unwrap()
            .value_y;
        println!("Shear(horizontal)): {} kN", shear / 1e3);
        assert_eq!(relative_eq!(shear, 0.0, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_axial_f_at_pl() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let p_load = Load::new_point_load(
            "Pointload".to_string(),
            "1".to_string(),
            "L/2".to_string(),
            "10".to_string(),
            0.0,
        );
        let loads = vec![p_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 1000.0)
            .unwrap()
            .value_y;
        println!("Axial force(1000): {} kN", axial_f / 1e3);
        assert_eq!(relative_eq!(axial_f, 5e3, epsilon = 1.0), true);
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000): {} kN", axial_f / 1e3);
        assert_eq!(relative_eq!(axial_f, -5e3, epsilon = 1.0), true);
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 3000.0)
            .unwrap()
            .value_y;
        println!("Axial force(3000): {} kN", axial_f / 1e3);
        assert_eq!(relative_eq!(axial_f, -5e3, epsilon = 1.0), true);

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000<-45): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(axial_f, -5e3 / 2f64.sqrt(), epsilon = 1.0),
            true
        );

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000<45): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(axial_f, -5e3 / 2f64.sqrt(), epsilon = 1.0),
            true
        );
    }

    #[test]
    fn t_calculate_axial_f_at_rl() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let r_load = Load::new_rotational_load(
            "RotationalLoad".to_string(),
            "1".to_string(),
            "L/2".to_string(),
            "10000000".to_string(),
        );
        let loads = vec![r_load];
        let structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 1000.0)
            .unwrap()
            .value_y;

        println!("Axial force(1999): {} kN", axial_f / 1e3);
        assert_eq!(relative_eq!(axial_f, 0e3, epsilon = 1.0), true);
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 1999.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000): {} kN", axial_f / 1e3);
        assert_eq!(relative_eq!(axial_f, 0e3, epsilon = 1.0), true);
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2001): {} kN", axial_f / 1e3);
        assert_eq!(relative_eq!(axial_f, 0e3, epsilon = 1.0), true);
    }

    #[test]
    fn t_calculate_axial_f_at_ll() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_line_load(
            "Lineload".to_string(),
            "1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            0.0,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 1000.0)
            .unwrap()
            .value_y;

        println!("Axial force(1000): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 - 10.0 * 1.0) * 1e3,
                epsilon = 1.0
            ),
            true
        );
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000): {} kN", axial_f / 1e3);
        assert_eq!(relative_eq!(axial_f, 0.0, epsilon = 1.0), true);
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 3000.0)
            .unwrap()
            .value_y;
        println!("Axial force(3000): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 - 10.0 * 3.0) * 1e3,
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 1000.0)
            .unwrap()
            .value_y;
        println!("Axial force(1000<-45): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 - 10.0 * 1.0) * 1e3 / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 1000.0)
            .unwrap()
            .value_y;
        println!("Axial force(1000<45): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 - 10.0 * 1.0) * 1e3 / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );
    }

    #[test]
    fn t_calculate_axial_f_at_tl_ltr_full() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_triangular_load(
            "TriangularLoad".to_string(),
            "1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            0.0,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 1000.0)
            .unwrap()
            .value_y;

        println!("Axial force(1000): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 * 2.0 / 3.0
                    - 10.0 * (4.0 - 1.0) / 4.0 * 1.0
                    - (10.0 - 10.0 * (4.0 - 1.0) / 4.0) * 1.0 / 2.0)
                    * 1e3,
                epsilon = 1.0
            ),
            true
        );
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 * 2.0 / 3.0
                    - 10.0 * (4.0 - 2.0) / 4.0 * 2.0
                    - (10.0 - 10.0 * (4.0 - 2.0) / 4.0) * 2.0 / 2.0)
                    * 1e3,
                epsilon = 1.0
            ),
            true
        );
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 3000.0)
            .unwrap()
            .value_y;
        println!("Axial force(3000): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 * 2.0 / 3.0
                    - 10.0 * (4.0 - 3.0) / 4.0 * 3.0
                    - (10.0 - 10.0 * (4.0 - 3.0) / 4.0) * 3.0 / 2.0)
                    * 1e3,
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000<-45): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 * 2.0 / 3.0
                    - 10.0 * (4.0 - 2.0) / 4.0 * 2.0
                    - (10.0 - 10.0 * (4.0 - 2.0) / 4.0) * 2.0 / 2.0)
                    * 1e3
                    / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000<45): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 * 2.0 / 3.0
                    - 10.0 * (4.0 - 2.0) / 4.0 * 2.0
                    - (10.0 - 10.0 * (4.0 - 2.0) / 4.0) * 2.0 / 2.0)
                    * 1e3
                    / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );
    }

    #[test]
    fn t_calculate_axial_f_at_tl_rtl_full() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let elements = vec![el];
        let l_load = Load::new_triangular_load(
            "TriangularLoad".to_string(),
            "1".to_string(),
            "L".to_string(),
            "0".to_string(),
            "10".to_string(),
            0.0,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];

        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 1000.0)
            .unwrap()
            .value_y;
        println!("Axial force(1000): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 * 1.0 / 3.0 - 10.0 * (1.0) / 4.0 * 1.0 / 2.0) * 1e3,
                epsilon = 1.0
            ),
            true
        );
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 * 1.0 / 3.0 - 10.0 * (2.0) / 4.0 * 2.0 / 2.0) * 1e3,
                epsilon = 1.0
            ),
            true
        );
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 3000.0)
            .unwrap()
            .value_y;
        println!("Axial force(3000): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 * 1.0 / 3.0 - 10.0 * (3.0) / 4.0 * 3.0 / 2.0) * 1e3,
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = -45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000<-45): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 * 1.0 / 3.0 - 10.0 * (2.0) / 4.0 * 2.0 / 2.0) * 1e3 / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );

        structure_model.loads[0].rotation = 45.0;

        let results =
            &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let axial_f = results.internal_force_results[&1]
            .get_force_at(ForceType::Axial, 2000.0)
            .unwrap()
            .value_y;
        println!("Axial force(2000<45): {} kN", axial_f / 1e3);
        assert_eq!(
            relative_eq!(
                axial_f,
                (10.0 * 4.0 / 2.0 * 1.0 / 3.0 - 10.0 * (2.0) / 4.0 * 2.0 / 2.0) * 1e3 / 2f64.sqrt(),
                epsilon = 1.0
            ),
            true
        );
    }

    #[test]
    fn t_get_elem_local_reactions() {
        let (elements, nodes) = common::get_structure_fem_matriisit();
        let loads = common::get_fem_matriisi_loads();
        let struct_model = StructureModel {
            nodes,
            elements: elements,
            loads: loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let calc_model = common::get_calc_model(&struct_model.elements, &struct_model.nodes);
        let results =
            vefem::fem::fem_handler::calculate(&struct_model, &mut EquationHandler::new());
        let calc_loads = vefem::loads::utils::extract_calculation_loads(
            &calc_model,
            &struct_model.loads,
            &LoadCombination::default(),
            &EquationHandler::new(),
        );
        let local_reactions = results[0].node_results.get_elem_local_nodal_force_vectors(
            &calc_model.get_all_calc_elements()[0],
            &calc_loads,
            &struct_model.calc_settings,
        );
        println!("Local reactions: {:.0}", local_reactions);
    }
}
