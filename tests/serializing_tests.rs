mod common;

#[cfg(test)]
pub mod serializing_tests {
    use std::collections::BTreeMap;

    use crate::common;
    use vefem::{
        loads::LoadCombination, material::{Concrete, MaterialData}, profile::Profile, reinforcement::{RebarCollection, RebarData, ReinforcementData}, settings::CalculationSettings, structure::{CalculationModel, Element, Node}, *
    };
    use vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};

    #[test]
    fn test_serializing() {
        let (elements, nodes) = common::get_structure_fem_matriisit_releases();
        let load_combinations: Vec<LoadCombination> = vec![];
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_model = CalculationModel {
            nodes,
            elements,
            load_combinations,
            loads,
            calc_settings,
        };
        let calc_model_json = serde_json::to_string_pretty(&calc_model).unwrap();
        println!("Calculation model JSON: {}", calc_model_json);
        let calc_model_deserialized: CalculationModel =
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

        let results1 = fem::calculate(&calc_model, &mut EquationHandler::new());
        let results2 = fem::calculate(&calc_model_deserialized, &mut EquationHandler::new());
        println!(
            "Support reaction node 1 dir 1: {:.2} = {:.2}",
            results1.node_results.get_support_reaction(1, 1),
            results2.node_results.get_support_reaction(1, 1)
        );
        assert_eq!(
            results1.node_results.displacements,
            results2.node_results.displacements
        );
        assert_eq!(
            results1.node_results.support_reactions,
            results2.node_results.support_reactions
        );

        let results_json = serde_json::to_string_pretty(&results2).unwrap();
        println!("Result JSON: {}", results_json.split_at_checked(1200).unwrap().0);
    }

    #[test]
    fn test_concrete_serializing() {
        let mut elements = vec![Element::new(1, 1, 2, Profile::new_rectangle("100x100".to_string(), 100.0, 100.0), 
            MaterialData::Concrete(Concrete::standard(material::StandardConcrete::C30_37)))];
        let nodes : BTreeMap<i32, Node> = BTreeMap::from([
            (1, Node::new_hinged(1, VpPoint::new(0.0, 0.0))),
            (2, Node::new_hinged(2, VpPoint::new(4000.0, 0.0)))
        ]);
        match &mut elements[0].material {
            MaterialData::Concrete(concrete) => {
                concrete.reinforcement.main_rebars.push(RebarCollection::new_bot_full(
                    ReinforcementData::Rebar(RebarData::new(500.0, 210e3)),
                    reinforcement::RebarDistribution::Even { diam: 20.0, count: 4, 
                        cc_left: "30".to_string(), cc_right: "30".to_string() },
                    "30".to_string()
                ))
            },
            MaterialData::Steel(steel) => panic!(),
            MaterialData::Timber(timber) => panic!(),
        }
        let load_combinations: Vec<LoadCombination> = vec![];
        let loads = common::get_fem_matriisi_loads();
        let calc_settings = CalculationSettings::default();
        let calc_model = CalculationModel {
            nodes,
            elements,
            load_combinations,
            loads,
            calc_settings,
        };
        let calc_model_json = serde_json::to_string_pretty(&calc_model).unwrap();
        println!("Calculation model JSON: {}", calc_model_json);
    }
}
