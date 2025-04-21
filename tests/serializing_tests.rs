mod common;

#[cfg(test)]
pub mod serializing_tests {
    use crate::common;
    use std::collections::BTreeMap;
    use std::ffi::{CStr, CString};
    use vefem::{
        loads::LoadCombination,
        material::{Concrete, MaterialData},
        profile::Profile,
        reinforcement::{RebarCollection, RebarData, ReinforcementData, ShearRebarGroup},
        settings::CalculationSettings,
        structure::{Element, Node, StructureModel},
        *,
    };
    use vputilslib::{
        equation_handler::EquationHandler,
        geometry2d::{Polygon, VpPoint},
    };

    #[test]
    fn test_serializing() {
        let (elements, nodes) = common::get_structure_fem_matriisit_releases();
        let load_combinations: Vec<LoadCombination> = vec![];
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_model = StructureModel {
            nodes,
            elements,
            load_combinations,
            loads,
            calc_settings,
        };
        let calc_model_json = serde_json::to_string_pretty(&calc_model).unwrap();
        println!("Calculation model JSON: {}", calc_model_json);
        let calc_model_deserialized: StructureModel =
            serde_json::from_str(&calc_model_json).unwrap();
        assert_eq!(
            calc_model.elements.len(),
            calc_model_deserialized.elements.len()
        );
        assert_eq!(calc_model.nodes.len(), calc_model_deserialized.nodes.len());
        assert_eq!(
            calc_model.load_combinations.len(),
            calc_model_deserialized.load_combinations.len()
        );
        assert_eq!(calc_model.loads.len(), calc_model_deserialized.loads.len());
        assert_eq!(
            calc_model.calc_settings.calc_split_interval,
            calc_model_deserialized.calc_settings.calc_split_interval
        );

        let results1 = fem::fem_handler::calculate(&calc_model, &mut EquationHandler::new());
        let results2 =
            fem::fem_handler::calculate(&calc_model_deserialized, &mut EquationHandler::new());
        println!(
            "Support reaction node 1 dir 1: {:.2} = {:.2}",
            results1[0].node_results.get_support_reaction(1, 1),
            results2[0].node_results.get_support_reaction(1, 1)
        );
        assert_eq!(
            results1[0].node_results.displacements,
            results2[0].node_results.displacements
        );
        assert_eq!(
            results1[0].node_results.support_reactions,
            results2[0].node_results.support_reactions
        );

        let results_json = serde_json::to_string_pretty(&results2).unwrap();
        println!(
            "Result JSON: {}",
            results_json.split_at_checked(1200).unwrap().0
        );
    }

    #[test]
    fn test_concrete_serializing() {
        let mut elements = vec![Element::new(
            1,
            1,
            2,
            Profile::new_rectangle("480x480".to_string(), 480.0, 480.0),
            MaterialData::Concrete(Concrete::standard(material::StandardConcrete::C30_37)),
        )];
        let nodes: BTreeMap<i32, Node> = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0))),
        ]);
        let prof_width = elements[0].profile.get_width();
        let prof_height = elements[0].profile.get_height();
        match &mut elements[0].material {
            MaterialData::Concrete(concrete) => {
                concrete
                    .reinforcement
                    .main_rebars
                    .push(RebarCollection::new_bot_full(
                        ReinforcementData::Rebar(RebarData::new(500.0, 200e3)),
                        reinforcement::RebarDistribution::Even {
                            diam: 20.0,
                            count: 4,
                            cc_start: "30".to_string(),
                            cc_end: "30".to_string(),
                        },
                        "30".to_string(),
                    ));
                concrete
                    .reinforcement
                    .shear_rebars
                    .push(ShearRebarGroup::new_full(
                        RebarData {
                            yield_strength: 500.0,
                            elastic_modulus: 200e3,
                        },
                        reinforcement::RebarDistribution::Spacing {
                            diam: 8.0,
                            spacing: 300.0,
                            cc_start: "150".to_string(),
                            cc_end: "150".to_string(),
                        },
                        Polygon::new(vec![
                            VpPoint::new(20.0, 20.0),
                            VpPoint::new(prof_width - 20.0, 20.0),
                            VpPoint::new(prof_width - 20.0, prof_height - 20.0),
                            VpPoint::new(20.0, prof_height - 20.0),
                            VpPoint::new(20.0, 20.0),
                        ]),
                    ));
            }
            MaterialData::Steel(_) => panic!(),
            MaterialData::Timber(_) => panic!(),
        }
        let load_combinations: Vec<LoadCombination> = vec![];
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_model = StructureModel {
            nodes,
            elements,
            load_combinations,
            loads,
            calc_settings,
        };
        let calc_model_json = serde_json::to_string_pretty(&calc_model).unwrap();
        println!("Calculation model JSON: {}", calc_model_json);
    }

    #[test]
    fn test_element_number_extracting() {
        let elements = vec![
            Element::new(
                1,
                1,
                2,
                common::get_default_profile(),
                common::get_default_material_steel(),
            ),
            Element::new(
                2,
                2,
                3,
                common::get_default_profile(),
                common::get_default_material_steel(),
            ),
            Element::new(
                3,
                3,
                4,
                common::get_default_profile(),
                common::get_default_material_steel(),
            )
        ];
        test_diff_load_element_strings(&elements, "-1");
        test_diff_load_element_strings(&elements, "1,3");
        test_diff_load_element_strings(&elements, "2");
        test_diff_load_element_strings(&elements, "1..2");
        test_diff_load_element_strings(&elements, "2..3");
        test_diff_load_element_strings(&elements, "1..3");
    }

    fn test_diff_load_element_strings(elements: &Vec<Element>, element_string: &str) {
        let load = common::get_default_line_load(element_string);
        let load_json = serde_json::to_string_pretty(&load).unwrap();
        let elements = serde_json::to_string_pretty(&elements).unwrap();
        let load_json = CString::new(load_json).unwrap().into_raw();
        let elements = CString::new(elements).unwrap().into_raw();
        let result = api::api_loads::extract_elements_from_load(load_json, elements);

        let result = unsafe { CStr::from_ptr(result).to_str().unwrap() };

        println!("{:?}", result);
    }
}
