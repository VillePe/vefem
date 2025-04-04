mod common;

#[cfg(test)]
mod tests {
    use crate::common;
    

    use vputilslib::equation_handler::EquationHandler;

    use vefem::{loads::{self, load_combination}, settings::CalculationSettings};

    #[test]
    fn test_get_loads() {
        let loads = common::get_fem_matriisi_loads();
        let (elements, nodes) = common::get_structure_fem_matriisit_releases();
        let mut load_combination = vefem::loads::LoadCombination::new(
            String::from("Load combination 1"), 
            load_combination::LoadCombinationType::ULS(false)
        );
        let mut load_combination2 = vefem::loads::LoadCombination::new(
            String::from("Load combination 2"), 
            load_combination::LoadCombinationType::ULS(false)
        );
        load_combination.add_load_n_factor(String::from("1"), 1.0);
        load_combination.add_load_n_factor(String::from("2"), 1.0);
        load_combination.add_load_n_factor(String::from("3"), 1.0);

        load_combination2.add_load_n_factor(String::from("1"), 1.2);
        load_combination2.add_load_n_factor(String::from("2"), 1.4);
        load_combination2.add_load_n_factor(String::from("3"), 2.0);

        let calc_loads = loads::utils::extract_calculation_loads(
            &elements, 
            &nodes,
            &loads,
            &load_combination2,
            &EquationHandler::new()
        );
        println!("{0} = {1}", calc_loads[0].name, calc_loads[0].strength);
        println!("{0} = {1}", calc_loads[1].name, calc_loads[1].strength);
        println!("{0} = {1}", calc_loads[2].name, calc_loads[2].strength);
        assert!(calc_loads[0].strength == 12.0); // 1
        assert!(calc_loads[1].strength == 14.0); // 2
        assert!(calc_loads[2].strength == 10.0); // 3

        let calc_model = vefem::structure::StructureModel{
            nodes,
            elements,            
            loads,
            load_combinations: vec![load_combination, load_combination2],
            calc_settings: CalculationSettings::default()
        };
        let results = vefem::fem::calculate(&calc_model, &EquationHandler::new());

        println!("{0}", results[0].load_combination);
        println!("{0}", results[0].node_results.support_reactions[0]); // Without load combination: 28.8e3
        println!("{0}", results[0].node_results.support_reactions[1]); // Without load combination: 30e3
        println!("{0}", results[0].node_results.support_reactions[2*3+0]); // Without load combination: 8.8e3
        println!("{0}", results[0].node_results.support_reactions[2*3+1]); // Without load combination: 30e3
        assert!((results[0].node_results.support_reactions[0] - (-28800.0)).abs() < 100.0);
        assert!((results[0].node_results.support_reactions[1] - (30000.0)).abs() < 10.0);
        assert!((results[0].node_results.support_reactions[2*3+0] - (8800.0)).abs() < 100.0);
        assert!((results[0].node_results.support_reactions[2*3+1] - (30000.0)).abs() < 10.0);

        println!("{0}", results[1].load_combination);
        println!("{0}", results[1].node_results.support_reactions[0]); // Without load combination: 28.8e3
        println!("{0}", results[1].node_results.support_reactions[1]); // Without load combination: 30e3
        println!("{0}", results[1].node_results.support_reactions[2*3+0]); // Without load combination: 8.8e3
        println!("{0}", results[1].node_results.support_reactions[2*3+1]); // Without load combination: 30e3
        assert!((results[1].node_results.support_reactions[0] - (-31500.0)).abs() < 10.0);
        assert!((results[1].node_results.support_reactions[1] - (42000.0)).abs() < 10.0);
        assert!((results[1].node_results.support_reactions[2*3+0] - (23500.0)).abs() < 10.0);
        assert!((results[1].node_results.support_reactions[2*3+1] - (42000.0)).abs() < 10.0);
    }
}