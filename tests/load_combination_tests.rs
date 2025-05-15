mod common;

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::common;

    use vputilslib::equation_handler::EquationHandler;

    use vefem::{loads::{self, load_combination}, settings::CalculationSettings};
    use vefem::loads::{Load, LoadGroup};

    #[test]
    fn test_get_loads() {
        let loads = common::get_fem_matriisi_loads();
        let (elements, nodes) = common::get_structure_fem_matriisit_releases();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let mut load_combination = vefem::loads::LoadCombination::new(
            1,
            String::from("Load combination 1"), 
            load_combination::LoadCombinationType::ULS{is_auto: false}
        );
        let mut load_combination2 = vefem::loads::LoadCombination::new(
            2,
            String::from("Load combination 2"), 
            load_combination::LoadCombinationType::ULS{is_auto: false}
        );
        load_combination.add_load_n_factor(String::from("1"), 1.0);
        load_combination.add_load_n_factor(String::from("2"), 1.0);
        load_combination.add_load_n_factor(String::from("3"), 1.0);

        load_combination2.add_load_n_factor(String::from("1"), 1.2);
        load_combination2.add_load_n_factor(String::from("2"), 1.4);
        load_combination2.add_load_n_factor(String::from("3"), 2.0);
        
        let calc_lc2 = &loads::lc_utils::get_calc_load_combinations(&load_combination2, &loads)[0];

        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads, 
            calc_lc2,
            &EquationHandler::new()
        );
        println!("{0} = {1}", calc_loads[0].name, calc_loads[0].strength);
        println!("{0} = {1}", calc_loads[1].name, calc_loads[1].strength);
        println!("{0} = {1}", calc_loads[2].name, calc_loads[2].strength);
        assert_eq!(calc_loads[0].strength, 12.0); // 1
        assert_eq!(calc_loads[1].strength, 14.0); // 2
        assert_eq!(calc_loads[2].strength, 10.0); // 3

        let calc_model = vefem::structure::StructureModel{
            nodes,
            elements,   
            loads,
            load_combinations: vec![load_combination, load_combination2],
            calc_settings: CalculationSettings::default()
        };
        let results = vefem::fem::fem_handler::calculate(&calc_model, &EquationHandler::new());

        println!("LC: {0}", results[0].load_combination);
        println!("SR0: {0}", results[0].node_results.support_reactions[0]); // Without load combination: 28.8e3
        println!("SR1: {0}", results[0].node_results.support_reactions[1]); // Without load combination: 30e3
        println!("SR6: {0}", results[0].node_results.support_reactions[2*3+0]); // Without load combination: 8.8e3
        println!("SR7: {0}", results[0].node_results.support_reactions[2*3+1]); // Without load combination: 30e3
        assert!((results[0].node_results.support_reactions[0] - (-28800.0)).abs() < 100.0);
        assert!((results[0].node_results.support_reactions[1] - (30000.0)).abs() < 10.0);
        assert!((results[0].node_results.support_reactions[2*3+0] - (8800.0)).abs() < 100.0);
        assert!((results[0].node_results.support_reactions[2*3+1] - (30000.0)).abs() < 10.0);

        println!("LC: {0}", results[1].load_combination);
        println!("SR0: {0}", results[1].node_results.support_reactions[0]); // Without load combination: 28.8e3
        println!("SR1: {0}", results[1].node_results.support_reactions[1]); // Without load combination: 30e3
        println!("SR6: {0}", results[1].node_results.support_reactions[2*3+0]); // Without load combination: 8.8e3
        println!("SR7: {0}", results[1].node_results.support_reactions[2*3+1]); // Without load combination: 30e3
        assert!((results[1].node_results.support_reactions[0] - (-31500.0)).abs() < 10.0);
        assert!((results[1].node_results.support_reactions[1] - (42000.0)).abs() < 10.0);
        assert!((results[1].node_results.support_reactions[2*3+0] - (23500.0)).abs() < 10.0);
        assert!((results[1].node_results.support_reactions[2*3+1] - (42000.0)).abs() < 10.0);
    }

    #[test]
    fn test_load_combination_with_ALL() {
        let loads = common::get_fem_matriisi_loads();
        let (elements, nodes) = common::get_structure_fem_matriisit_releases();
        let calc_model = common::get_calc_model(&elements, &nodes);
        let mut load_combination = vefem::loads::LoadCombination::new(
            1,
            String::from("Load combination 1"),
            load_combination::LoadCombinationType::ULS{is_auto: false}
        );
        load_combination.add_load_n_factor(String::from("ALL"), 1.0);

        let calc_lc = &loads::lc_utils::get_calc_load_combinations(&load_combination, &loads)[0];

        let calc_loads = loads::utils::extract_calculation_loads(
            &calc_model,
            &loads,
            calc_lc,
            &EquationHandler::new()
        );
        println!("{0} = {1}", calc_loads[0].name, calc_loads[0].strength);
        println!("{0} = {1}", calc_loads[1].name, calc_loads[1].strength);
        println!("{0} = {1}", calc_loads[2].name, calc_loads[2].strength);
        assert_eq!(calc_loads[0].strength, 10.0); // 1
        assert_eq!(calc_loads[1].strength, 10.0); // 2
        assert_eq!(calc_loads[2].strength, 5.0); // 3

        let calc_model = vefem::structure::StructureModel{
            nodes,
            elements,
            loads,
            load_combinations: vec![load_combination],
            calc_settings: CalculationSettings::default()
        };
        let results = vefem::fem::fem_handler::calculate(&calc_model, &EquationHandler::new());

        println!("LC: {0}", results[0].load_combination);
        println!("SR0: {0}", results[0].node_results.support_reactions[0]); 
        println!("SR1: {0}", results[0].node_results.support_reactions[1]); 
        println!("SR6: {0}", results[0].node_results.support_reactions[2*3+0]);
        println!("SR7: {0}", results[0].node_results.support_reactions[2*3+1]);
        assert!((results[0].node_results.support_reactions[0] - (-28750.0)).abs() < 100.0);
        assert!((results[0].node_results.support_reactions[1] - (30000.0)).abs() < 10.0);
        assert!((results[0].node_results.support_reactions[2*3+0] - (8750.0)).abs() < 100.0);
        assert!((results[0].node_results.support_reactions[2*3+1] - (30000.0)).abs() < 10.0);
    }

    #[test]
    fn test_load_combination_with_ALL_auto() {
        let mut loads : Vec<Load> = Vec::new();
        loads.push(Load::new_line_load(
           "perm".to_string(),
           "-1".to_string(),
           "0".to_string(),
           "L".to_string(),
           "10".to_string(),
           -90.0,
           LoadGroup::PERMANENT
        ));
        loads.push(Load::new_line_load(
            "ClassA".to_string(),
            "-1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
            LoadGroup::CLASS_A
        ));
        loads.push(Load::new_line_load(
            "ClassB".to_string(),
            "-1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
            LoadGroup::CLASS_B
        ));
        let (elements, nodes) = common::get_structure_three_horizontal_elements();
        let mut load_combination = vefem::loads::LoadCombination::new(
            1,
            String::from("Load combination 1"),
            load_combination::LoadCombinationType::ULS{is_auto: true}
        );
        load_combination.add_load_n_factor(String::from("ALL"), 1.0);

        let calc_model = vefem::structure::StructureModel{
            nodes,
            elements,
            loads,
            load_combinations: vec![load_combination],
            calc_settings: CalculationSettings::default()
        };
        let results = vefem::fem::fem_handler::calculate(&calc_model, &EquationHandler::new());
        println!("Results count: {0}", results.len());

        println!("LC: {0}", results[0].load_combination);
        println!("SR1: {0}", results[0].node_results.support_reactions[1]);
        println!("SR7: {0}", results[0].node_results.support_reactions[1*3+1]);
        assert!((results[0].node_results.support_reactions[1] - (21600.0)).abs() < 10.0);
        assert!((results[0].node_results.support_reactions[1*3+1] - (59400.0)).abs() < 10.0);
    }
}