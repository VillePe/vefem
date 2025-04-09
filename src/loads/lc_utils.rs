use std::collections::BTreeMap;

use crate::settings::CalculationSettings;

use super::{
    load::Load, load_combination::CalcLoadCombination, load_group::GroupType, LoadCombination,
    LoadGroup,
};

pub fn get_calc_load_combinations(
    lc: &LoadCombination,
    loads: &Vec<Load>,
) -> Vec<CalcLoadCombination> {
    let mut result: Vec<CalcLoadCombination> = Vec::new();
    let loads_map = crate::loads::utils::get_load_map(&loads);

    let mut permanents_only = CalcLoadCombination::new(
        lc.name.clone(),
        "_PERM".to_string(),
        lc.combination_type,
        BTreeMap::new(),
    );
    for load_name in loads_map.keys() {
        if !load_is_included(lc, &load_name) {
            continue;
        }
        
        let loads = loads_map.get(load_name).unwrap();
        for load in loads {
            match load.load_group.group_type {
                GroupType::Permanent => {
                    permanents_only
                        .loads_n_factors
                        .insert(load.name.clone(), lc.loads_n_factors[load_name] * 1.35);
                }
                _ => (),
            }
        }
    }

    result.push(permanents_only);

    // Gather the loads by group into a map
    let mut loads_mapped_by_group: BTreeMap<&LoadGroup, Vec<&Load>> = BTreeMap::new();
    for load in loads.iter() {
        if !loads_mapped_by_group.contains_key(&load.load_group) {
            loads_mapped_by_group.insert(&load.load_group, Vec::new());
        }
        loads_mapped_by_group
            .get_mut(&load.load_group)
            .unwrap()
            .push(load);
    }

    // Go through the groups and create load combinations with one group as the 'main' group
    // and others the 'secondary' groups (1.15 * Gk + 1.5 * Qk,1 + sum(1,5 * ψ0,i * Qk,i))
    for group in loads_mapped_by_group.keys() {
        let mut calc_lc = CalcLoadCombination::new(
            lc.name.clone(),
            format!("_LL{}", group.get_name()),
            lc.combination_type,
            BTreeMap::new(),
        );

        // Iterate through all load names
        for load_name in loads_map.keys() {
            // If the load is not included in the parent load combination, don't add it into calculation combinations
            if !lc.loads_n_factors.contains_key(load_name) {
                continue;
            }
            // Get the loads with the current load name
            let loads = loads_map.get(load_name).unwrap();
            for load in loads {
                match load.load_group.group_type {
                    GroupType::Permanent => {
                        calc_lc.loads_n_factors.insert(
                            load.name.clone(),
                            lc.loads_n_factors[load_name] * load.load_group.uls_factor,
                        );
                    }
                    GroupType::PermanentFav => {
                        calc_lc.loads_n_factors.insert(
                            load.name.clone(),
                            lc.loads_n_factors[load_name] * load.load_group.uls_factor,
                        );
                    }
                    GroupType::LiveLoad => {
                        // If the current load is in the group that is currently the 'main' group,
                        // add it with the unmodified uls factor. Otherwise modify the uls factor with ψ0
                        // of the loads group
                        if *group == &load.load_group {
                            calc_lc.loads_n_factors.insert(
                                load.name.clone(),
                                lc.loads_n_factors[load_name] * load.load_group.uls_factor,
                            );
                        } else {
                            calc_lc.loads_n_factors.insert(
                                load.name.clone(),
                                lc.loads_n_factors[load_name]
                                    * load.load_group.uls_factor
                                    * load.load_group.psii0,
                            );
                        }
                    }
                    _ => continue,
                }
            }
        }
    }

    result
}

pub fn load_is_included(lc: &LoadCombination, load_name: &str) -> bool {
    if lc.loads_n_factors.is_empty() || lc.loads_n_factors.contains_key("ALL") {
        return true;
    }
    lc.loads_n_factors.contains_key(load_name)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::loads::load_group::*;
    use crate::loads::load::*;
    use crate::loads::load_combination::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_get_calc_load_combinations() {
        let mut loads = Vec::new();
        let g_oma = Load::new_line_load(
            "g_oma".to_string(),
            "1".to_string(),
            "0.0".to_string(),
            "10.0".to_string(),
            "1000.0".to_string(),
            0.0,
            LoadGroup::PERMANENT,
        );
        let g = Load::new_line_load(
            "g".to_string(),
            "1".to_string(),
            "0.0".to_string(),
            "10.0".to_string(),
            "1000.0".to_string(),
            0.0,
            LoadGroup::PERMANENT,
        );
        let q1 = Load::new_line_load(
            "q1".to_string(),
            "1".to_string(),
            "0.0".to_string(),
            "10.0".to_string(),
            "1000.0".to_string(),
            0.0,
            LoadGroup::CLASS_A,
        );
        let qs = Load::new_line_load(
            "qs".to_string(),
            "1".to_string(),
            "0.0".to_string(),
            "10.0".to_string(),
            "1000.0".to_string(),
            0.0,
            LoadGroup::SNOW,
        );
        let qw = Load::new_line_load(
            "qw".to_string(),
            "1".to_string(),
            "0.0".to_string(),
            "10.0".to_string(),
            "1000.0".to_string(),
            0.0,
            LoadGroup::WIND_POS,
        );

        loads.push(g_oma);
        loads.push(g);
        loads.push(q1);
        loads.push(qs);
        loads.push(qw);

        let lc = LoadCombination {
            name: "TEST".to_string(),
            loads_n_factors: BTreeMap::from([
                ("g_oma".to_string(), 1.0),
                ("g".to_string(), 1.0),
                ("q1".to_string(), 1.0),
                ("qs".to_string(), 1.0),
                ("qw".to_string(), 1.0),
            ]),
            combination_type: LoadCombinationType::ULS { is_auto: true },
        };

        let result = get_calc_load_combinations(&lc, &loads);
        for r in result.iter() {
            println!("{}", r.sub_name);
        }
        assert_eq!(result.len(), 4);
    }
}
