use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadCombination {
    /// The name of the load combination. The name has to be unique (except for default load combination,
    /// which has the name of 'DEFAULT_ALL_LOADS_COMBINATION').
    pub name: String,
    /// The combination type
    pub combination_type: LoadCombinationType,
    /// A map of load names and their factors. This map controls which loads are included in the calculation
    /// If the map is empty, all loads are calculated as such.
    pub loads_n_factors: BTreeMap<String, f64>,    
}
impl LoadCombination {
    const DEFAULT_NAME : &'static str = "DEFAULT_ALL_LOADS_COMBINATION";

    pub fn new(name: String, combination_type: LoadCombinationType) -> Self {
        Self { 
            name, 
            combination_type, 
            loads_n_factors: BTreeMap::new() 
        }
    }
    
    /// Adds the tuple of string and f64 which represent a load name and the factor for the loads
    /// with that name.
    pub fn add_load_n_factor(&mut self, load_name: String, load_factor: f64) {
        self.loads_n_factors.insert(load_name, load_factor);
    }
}

impl Default for LoadCombination {
    fn default() -> Self {
        Self { name: LoadCombination::DEFAULT_NAME.to_string(), 
        combination_type: LoadCombinationType::None, 
        loads_n_factors: Default::default() }
    }
}

/// The load combination type. All types have a bool parameter to control whether the load combinations
/// should be automatically created (when 'exploding' the load combination).
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$type")]
pub enum LoadCombinationType {
    /// Ultimate limit state. The bool indicates if the load combination should be automatically
    /// created by load groups.
    ULS{is_auto: bool},
    /// Serviceability limit state - characteristic. Should be used when the material calculated
    /// behaves elastically (doesn't return to its original state after unloading, like cracked concrete).
    /// The bool indicates if the load combination should be automatically created by load groups.
    SLSc{is_auto: bool},
    /// Serviceability limit state - frequent. Should be used when the material calculated
    /// behaves elastically (returns to its original state after unloading, like steel).
    /// The bool indicates if the load combination should be automatically created by load groups.
    SLSf{is_auto: bool},
    /// Serviceability limit state - quasi-permanent. Should be used when calculating the
    /// quasi-permanent effects of loading (such as creep on concrete).
    /// The bool indicates if the load combination should be automatically created by load groups.
    SLSqp{is_auto: bool},
    None,
}