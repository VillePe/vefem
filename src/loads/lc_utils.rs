use super::{load::{self, CalculationLoad}, utils::get_load_map, LoadCombination};



pub fn get_loads(load_combination: &LoadCombination, original_loads: Vec<load::CalculationLoad>) -> Vec<load::CalculationLoad> {
    let mut result = Vec::new();
    let load_map = get_load_map(original_loads);
    for tuple in load_combination.loads_n_factors.iter() {
        let load_name = tuple.0;
        let load_factor = tuple.1;
        println!("Load name: {load_name}");
        if load_map.contains_key(load_name) {
            println!("Load name: {load_name}");
            for load in load_map[load_name].iter() {
                // Copy the calculation load object and modify the strength
                let mut modified_load = CalculationLoad{
                    name: load.name.clone(),
                    .. *load
                };
                modified_load.strength = modified_load.strength * load_factor;
                result.push(modified_load);
            }            
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};

    use crate::{loads::{self, load::{self, CalculationLoad, CalculationLoadType}, load_combination}, material::Concrete, structure::{self, Node, Support}};

    #[test]
    fn test_get_loads() {
        let load1 = load::Load::new_line_load(
            "g_oma".to_string(),
            "-1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "5".to_string(),
            -90.0,
        );
        let load2 = load::Load::new_line_load(
            "g".to_string(),
            "-1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
        );
        let load3 = load::Load::new_line_load( 
            "g".to_string(),
            "2".to_string(), // For this line should not be included (unless element 2 is added)
            "0".to_string(),
            "L".to_string(),
            "20".to_string(),
            -90.0,
        );
        let load4 = load::Load::new_line_load(
            "q".to_string(),
            "-1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
        );
        let load5 = load::Load::new_line_load(
            "qs".to_string(),
            "-1".to_string(),
            "0".to_string(),
            "L".to_string(),
            "10".to_string(),
            -90.0,
        );
        let mut load_combination = super::LoadCombination::new(
            String::from("Load combination 1"), 
            load_combination::LoadCombinationType::ULS(false)
        );
        load_combination.add_load_n_factor(String::from("g_oma"), 1.15);
        load_combination.add_load_n_factor(String::from("g"), 1.15);
        load_combination.add_load_n_factor(String::from("q"), 1.05);
        load_combination.add_load_n_factor(String::from("qs"), 1.5);
        let original_loads = vec![load1, load2, load3, load4, load5];

        let el = structure::Element::new(1, 1, 2, crate::profile::Profile::new_rectangle("".to_string(), 100.0, 100.0), 
            crate::material::MaterialData::Concrete(Concrete::standard(crate::material::StandardConcrete::C30_37))
        );
        let n1 = Node::new(1, VpPoint::new(0.0, 0.0), Support::new_hinged());
        let n2 = Node::new(2, VpPoint::new(4000.0, 0.0), Support::new_hinged());

        let calc_loads = loads::utils::extract_calculation_loads(
            &vec![el], 
            &BTreeMap::from([(1,n1), (2,n2)]), 
            &original_loads,
            &EquationHandler::new()
        );
        let calc_loads = super::get_loads(&load_combination, calc_loads);
        println!("{0} = {1}", calc_loads[0].name, calc_loads[0].strength);
        println!("{0} = {1}", calc_loads[1].name, calc_loads[1].strength);
        println!("{0} = {1}", calc_loads[2].name, calc_loads[2].strength);
        println!("{0} = {1}", calc_loads[3].name, calc_loads[3].strength);
        assert!(calc_loads[0].strength == 11.5); // g
        assert!(calc_loads[1].strength == 5.75); // g_oma
        assert!(calc_loads[2].strength == 10.5); // q
        assert!(calc_loads[3].strength == 15.0); // qs
    }
}
