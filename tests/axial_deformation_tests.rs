mod common;

#[cfg(test)]
mod axial_deformation_tests {
    
    use std::collections::BTreeMap;

    use approx::relative_eq;
    use vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};

    use vefem::{
        loads::{Load, LoadGroup},
        material::{MaterialData, Steel},
        profile::Profile,
        results::ForceType,
        settings::CalculationSettings,
        structure::{Element, Node, StructureModel},
    };

    #[test]
    fn t_calculate_axial_deformation_at_pl_1() {
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
            "1000".to_string(),
            0.0,
            LoadGroup::PERMANENT,
        );
        let loads = vec![p_load];
        let mut struct_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let results = &vefem::fem::fem_handler::calculate(&struct_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 1000.0)
            .unwrap()
            .value_x;
        println!("Strain(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.238, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.476, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 3000.0)
            .unwrap()
            .value_x;
        println!("Strain(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.238, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 4000.0)
            .unwrap()
            .value_x;
        println!("Strain(4000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.0, epsilon = 0.01), true);

        struct_model.loads[0].rotation = -45.0;
        let results = &vefem::fem::fem_handler::calculate(&struct_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.337, epsilon = 0.01), true);

        struct_model.loads[0].rotation = 135.0;
        let results = &vefem::fem::fem_handler::calculate(&struct_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, -0.337, epsilon = 0.01), true);
    }

    #[test]
    fn t_calculate_axial_deformation_at_pl_2() {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(2828.427, 2828.427))),
        ]);
        let elements = vec![el];
        let p_load = Load::new_point_load(
            "Pointload".to_string(),
            "1".to_string(),
            "L/2".to_string(),
            "1000".to_string(),
            45.0,
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
        let results = &vefem::fem::fem_handler::calculate(&struct_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 1000.0)
            .unwrap()
            .value_x;
        println!("Strain(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.238, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000): {} mm", defl);
    }

    #[test]
    fn t_calculate_axial_deformation_at_ll() {
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
            "1000".to_string(),
            0.0,
            LoadGroup::PERMANENT,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        println!("LENGTH: {:?}", results.internal_force_results.len());
        println!("LENGTH2: {:?}", results.internal_force_results[&1].deflections.len());
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 1000.0)
            .unwrap()
            .value_x;
        println!("Strain(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.714, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.952, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 3000.0)
            .unwrap()
            .value_x;
        println!("Strain(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.714, epsilon = 0.01), true);

        structure_model.loads[0].rotation = -45.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.673, epsilon = 0.01), true);

        structure_model.loads[0].rotation = -45.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.673, epsilon = 0.01), true);

        structure_model.loads[0].offset_start = "500".to_string();
        structure_model.loads[0].offset_end = "1500".to_string();
        structure_model.loads[0].rotation = 0.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(slice): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.238, epsilon = 0.01), true);
    }

    #[test]
    fn t_calculate_axial_deformation_at_tl_ltr_full() {
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
            "1000".to_string(),
            0.0,
            LoadGroup::PERMANENT,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new());
        println!("Results: {results:?}");
        let results = &results[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 1000.0)
            .unwrap()
            .value_x;
        println!("Strain(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.417, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.476, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 3000.0)
            .unwrap()
            .value_x;
        println!("Strain(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.298, epsilon = 0.01), true);

        structure_model.loads[0].rotation = -45.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.337, epsilon = 0.01), true);

        structure_model.loads[0].rotation = 135.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, -0.337, epsilon = 0.01), true);
    }

    /// Slice where load ends before x
    #[test]
    fn t_calculate_axial_deformation_at_tl_ltr_slice() {
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
            "500".to_string(),
            "1500".to_string(),
            "1000".to_string(),
            0.0,
            LoadGroup::PERMANENT,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.099, epsilon = 0.1), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 3000.0)
            .unwrap()
            .value_x;
        println!("Strain(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.05, epsilon = 0.1), true);

        structure_model.loads[0].rotation = -45.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.07, epsilon = 0.01), true);

        structure_model.loads[0].rotation = 135.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, -0.07, epsilon = 0.01), true);
    }

    #[test]
    fn t_calculate_axial_deformation_at_tl_rtl_full() {
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
            "1000".to_string(),
            0.0,
            LoadGroup::PERMANENT,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 1000.0)
            .unwrap()
            .value_x;
        println!("Strain(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.298, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.476, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 3000.0)
            .unwrap()
            .value_x;
        println!("Strain(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.417, epsilon = 0.01), true);

        structure_model.loads[0].rotation = -45.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.337, epsilon = 0.01), true);

        structure_model.loads[0].rotation = 135.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, -0.337, epsilon = 0.01), true);
    }

    /// Slice where load ends before x
    #[test]
    fn t_calculate_axial_deformation_at_tl_rtl_slice() {
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
            "1500".to_string(),
            "500".to_string(),
            "1000".to_string(),
            0.0,
            LoadGroup::PERMANENT,
        );
        let loads = vec![l_load];
        let mut structure_model = StructureModel {
            nodes,
            elements,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.139, epsilon = 0.1), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 3000.0)
            .unwrap()
            .value_x;
        println!("Strain(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.069, epsilon = 0.1), true);

        structure_model.loads[0].rotation = -45.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<-45): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.098, epsilon = 0.01), true);

        structure_model.loads[0].rotation = 135.0;
        let results = &vefem::fem::fem_handler::calculate(&structure_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000<45): {} mm", defl);
        assert_eq!(relative_eq!(defl, -0.098, epsilon = 0.01), true);
    }
    
    fn get_struct_for_strain_test(start_node: Node, end_node: Node) -> StructureModel {
        let el: Element = Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
            MaterialData::Steel(Steel::default()),
        );
        let nodes = BTreeMap::from([
            (start_node.number, start_node),
            (end_node.number, end_node),
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
        struct_model
    }

    #[test]
    fn t_calculate_axial_deformation_at_strain_1() {
        let struct_model = get_struct_for_strain_test(
            Node::new_fixed(1, VpPoint::new(0.0, 0.0)),
            Node::new_free(2, VpPoint::new(4000.0, 0.0)),
        );
        let results = &vefem::fem::fem_handler::calculate(&struct_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 0.0)
            .unwrap()
            .value_x;
        println!("Strain(0): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.0, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 1000.0)
            .unwrap()
            .value_x;
        println!("Strain(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -2.5, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -5.0, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 3000.0)
            .unwrap()
            .value_x;
        println!("Strain(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -7.5, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 4000.0)
            .unwrap()
            .value_x;
        println!("Strain(4000): {} mm", defl);
        assert_eq!(relative_eq!(defl, -10.0, epsilon = 0.01), true);
    }

    #[test]
    fn t_calculate_axial_deformation_at_strain_2() {
        let struct_model = get_struct_for_strain_test(
            Node::new_free(1, VpPoint::new(0.0, 0.0)),
            Node::new_fixed(2, VpPoint::new(4000.0, 0.0)),
        );
        let results = &vefem::fem::fem_handler::calculate(&struct_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 0.0)
            .unwrap()
            .value_x;
        println!("Strain(0): {} mm", defl);
        assert_eq!(relative_eq!(defl, 10.0, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 1000.0)
            .unwrap()
            .value_x;
        println!("Strain(1000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 7.5, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 5.0, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 3000.0)
            .unwrap()
            .value_x;
        println!("Strain(3000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 2.5, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 4000.0)
            .unwrap()
            .value_x;
        println!("Strain(4000): {} mm", defl);
        assert_eq!(relative_eq!(defl, 0.0, epsilon = 0.01), true);
    }

    #[test]
    fn t_calculate_axial_deformation_at_strain_3() {
        let struct_model = get_struct_for_strain_test(
            Node::new_free(1, VpPoint::new(0.0, 0.0)),
            Node::new_fixed(2, VpPoint::new(2828.57, 2828.57)),
        );
        println!("{}", struct_model.elements[0].get_length(&struct_model.nodes));
        let results = &vefem::fem::fem_handler::calculate(&struct_model, &mut EquationHandler::new())[0];
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 0.0)
            .unwrap()
            .value_x;
        println!("Strain(0): {} mm", defl);
        assert_eq!(relative_eq!(defl, 10.0, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 1000.0)
            .unwrap()
            .value_x;
        println!("Strain(1000): {} mm", defl);
        // assert_eq!(relative_eq!(defl, 7.5, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 2000.0)
            .unwrap()
            .value_x;
        println!("Strain(2000): {} mm", defl);
        // assert_eq!(relative_eq!(defl, 5.0, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 3000.0)
            .unwrap()
            .value_x;
        println!("Strain(3000): {} mm", defl);
        // assert_eq!(relative_eq!(defl, 2.5, epsilon = 0.01), true);
        let defl = results.internal_force_results[&1]
            .get_force_at(ForceType::Deflection, 4000.0)
            .unwrap()
            .value_x;
        println!("Strain(4000): {} mm", defl);
        // assert_eq!(relative_eq!(defl, 0.0, epsilon = 0.01), true);
    }
}
