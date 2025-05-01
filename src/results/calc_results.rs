use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};

use super::{InternalForceResults, NodeResults};

#[derive(Serialize, Deserialize)]
pub struct CalculationResults {
    pub load_combination: String,
    pub load_comb_num: usize,
    pub sub_load_comb_num: usize,
    pub node_results: NodeResults,
    pub internal_force_results: BTreeMap<i32, InternalForceResults>,
}

impl Debug for CalculationResults {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CalculationResults, lc: {}, lc_num: {}, sub_lc_num: {}", self.load_combination, self.load_comb_num, self.sub_load_comb_num)
    }
}