use std::collections::BTreeMap;
use super::{
    load::Load, load_combination::CalcLoadCombination, load_group::GroupType, LoadCombination,
    LoadGroup,
};

pub fn get_calc_load_combinations(
    lc: &LoadCombination,
    loads: &Vec<Load>,
) -> Vec<CalcLoadCombination> {
    let mut result: Vec<CalcLoadCombination> = Vec::new();
        
    // If the load combination type is not auto, create only one load combination
    // which is the same as the original load combination
    if !lc.combination_type.is_auto() {
        let mut calc_lc = CalcLoadCombination::new(
            lc.number,
            lc.name.clone(),
            lc.number*1000+1,
            "".to_string(),
            lc.combination_type,
        );
        for l_n_f in lc.loads_n_factors.iter() {
            calc_lc.loads_n_factors.insert(l_n_f.0.clone(), *l_n_f.1);
        }
        
        result.push(calc_lc);
        return result;
    }
    
    let loads_map = crate::loads::utils::get_load_map(&loads);

    let mut permanents_only = CalcLoadCombination::new(
        lc.number,
        lc.name.clone(),
        lc.number*1000+1,
        "_PERM".to_string(),
        lc.combination_type,
    );
    for load_name in loads_map.keys() {
        if !load_is_included(lc, &load_name) {
            continue;
        }
        
        let loads = loads_map.get(load_name).unwrap();
        for load in loads {
            let factor = if lc.loads_n_factors.contains_key("ALL") {
                lc.loads_n_factors["ALL"]
            } else if lc.loads_n_factors.contains_key(load_name) {
                lc.loads_n_factors[load_name]
            } else {
                1.0
            };
            match load.load_group.group_type {
                GroupType::Permanent => {
                    permanents_only
                        .loads_n_factors
                        .insert(load.name.clone(), factor * 1.35);
                }
                _ => (),
            }
        }
    }

    result.push(permanents_only);

    // Create a map of groups from all loads that are included in the load combination
    let mut loads_mapped_by_group: BTreeMap<&LoadGroup, Vec<&Load>> = BTreeMap::new();
    for load in loads.iter() {
        if !load_is_included(lc, &load.name) {
            continue;
        }
        if !loads_mapped_by_group.contains_key(&load.load_group) {
            loads_mapped_by_group.insert(&load.load_group, Vec::new());
        }
        loads_mapped_by_group
            .get_mut(&load.load_group)
            .unwrap()
            .push(load);
    }

    let mut combination_number = lc.number*1000+2;
    // Go through the groups and create load combinations with one group as the 'main' group
    // and others the 'secondary' groups (1.15 * Gk + 1.5 * Qk,1 + sum(1,5 * ψ0,i * Qk,i))
    for group in loads_mapped_by_group.keys() {
        if matches!(group.group_type, GroupType::Permanent) || matches!(group.group_type, GroupType::PermanentFav) {
            continue;
        }
        let mut calc_lc = CalcLoadCombination::new(
            lc.number,
            lc.name.clone(),
            combination_number,
            format!("_LL({})", group.get_name()),
            lc.combination_type,
        );
        combination_number = combination_number+1;

        // Iterate through all load names
        for load_name in loads_map.keys() {
            // If the load is not included in the parent load combination, don't add it into calculation combinations
            if !load_is_included(lc, load_name) {
                continue;
            }
            let factor = if lc.loads_n_factors.contains_key(load_name) {
                lc.loads_n_factors[load_name]
            } else {
                1.0
            };
            // Get the loads with the current load name
            let loads = loads_map.get(load_name).unwrap();
            for load in loads {
                match load.load_group.group_type {
                    GroupType::Permanent => {
                        calc_lc.loads_n_factors.insert(
                            load.name.clone(),
                            factor * load.load_group.uls_factor,
                        );
                    }
                    GroupType::PermanentFav => {
                        calc_lc.loads_n_factors.insert(
                            load.name.clone(),
                            factor * load.load_group.uls_factor,
                        );
                    }
                    GroupType::LiveLoad => {
                        // If the current load is in the group that is currently the 'main' group,
                        // add it with the unmodified uls factor. Otherwise modify the uls factor with ψ0
                        // of the loads group
                        if *group == &load.load_group {
                            calc_lc.loads_n_factors.insert(
                                load.name.clone(),
                                factor * load.load_group.uls_factor,
                            );
                        } else {
                            calc_lc.loads_n_factors.insert(
                                load.name.clone(),
                                factor
                                    * load.load_group.uls_factor
                                    * load.load_group.psii0,
                            );
                        }
                    }
                }
            }
        }
        if !calc_lc.loads_n_factors.is_empty() {
            result.push(calc_lc);
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

pub fn calc_load_is_included(lc: &CalcLoadCombination, load_name: &str) -> bool {
    if lc.loads_n_factors.is_empty() || lc.loads_n_factors.contains_key("ALL") {
        return true;
    }
    lc.loads_n_factors.contains_key(load_name)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::loads::load::*;
    use crate::loads::load_combination::*;
    use crate::loads::load_group::*;
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
            number: 1,
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
        test_load_combination(lc, &loads, 4);

        let lc = LoadCombination {
            number: 1,
            name: "TEST".to_string(),
            loads_n_factors: BTreeMap::from([
                ("g_oma".to_string(), 1.0),
            ]),
            combination_type: LoadCombinationType::ULS { is_auto: true },
        };
        test_load_combination(lc, &loads, 1);

        let lc = LoadCombination {
            number: 1,
            name: "TEST".to_string(),
            loads_n_factors: BTreeMap::from([
                ("g_oma".to_string(), 1.0),
                ("g".to_string(), 1.0),
            ]),
            combination_type: LoadCombinationType::ULS { is_auto: true },
        };
        test_load_combination(lc, &loads, 1);

        let lc = LoadCombination {
            number: 1,
            name: "TEST".to_string(),
            loads_n_factors: BTreeMap::from([
                ("g_oma".to_string(), 1.0),
                ("g".to_string(), 1.0),
                ("q1".to_string(), 1.0),
            ]),
            combination_type: LoadCombinationType::ULS { is_auto: true },
        };
        test_load_combination(lc, &loads, 2);

        let lc = LoadCombination {
            number: 1,
            name: "TEST".to_string(),
            loads_n_factors: BTreeMap::from([
                ("g_oma".to_string(), 1.0),
                ("g".to_string(), 1.0),
                ("q1".to_string(), 1.0),
                ("qs".to_string(), 1.0),
            ]),
            combination_type: LoadCombinationType::ULS { is_auto: true },
        };
        test_load_combination(lc, &loads, 3);

        let lc = LoadCombination {
            number: 1,
            name: "TEST".to_string(),
            loads_n_factors: BTreeMap::from([
                ("ALL".to_string(), 1.0),
            ]),
            combination_type: LoadCombinationType::ULS { is_auto: true },
        };
        test_load_combination(lc, &loads, 4);

        let lc = LoadCombination {
            number: 1,
            name: "TEST".to_string(),
            loads_n_factors: BTreeMap::from([
            ]),
            combination_type: LoadCombinationType::ULS { is_auto: true },
        };
        test_load_combination(lc, &loads, 4);
    }

    fn test_load_combination(lc: LoadCombination, loads: &Vec<Load>, assert_count: usize) {
        let result = get_calc_load_combinations(&lc, &loads);
        for r in result.iter() {
            println!("{}", r.sub_name);
        }
        assert_eq!(result.len(), assert_count);
        println!()
    }
}
