mod common;

#[cfg(test)]
mod tests {
    use crate::common;
    

    use vputilslib::equation_handler::EquationHandler;

    use vefem::loads::{self, load_combination};

    #[test]
    fn test_get_loads() {
        let loads = common::get_fem_matriisi_loads();
        let (elements, nodes) = common::get_structure_fem_matriisit();
        let mut load_combination = vefem::loads::LoadCombination::new(
            String::from("Load combination 1"), 
            load_combination::LoadCombinationType::ULS(false)
        );
        load_combination.add_load_n_factor(String::from("1"), 1.1);
        load_combination.add_load_n_factor(String::from("2"), 1.2);
        load_combination.add_load_n_factor(String::from("3"), 2.0);

        let calc_loads = loads::utils::extract_calculation_loads(
            &elements, 
            &nodes,
            &loads,
            &load_combination,
            &EquationHandler::new()
        );
        println!("{0} = {1}", calc_loads[0].name, calc_loads[0].strength);
        println!("{0} = {1}", calc_loads[1].name, calc_loads[1].strength);
        println!("{0} = {1}", calc_loads[2].name, calc_loads[2].strength);
        assert!(calc_loads[0].strength == 11.0); // 1
        assert!(calc_loads[1].strength == 12.0); // 2
        assert!(calc_loads[2].strength == 10.0); // 3
    }
}