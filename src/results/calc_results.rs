use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{InternalForceResults, NodeResults};

#[derive(Serialize, Deserialize)]
pub struct CalculationResults {
    pub load_combination: String,
    pub node_results: NodeResults,
    pub internal_force_results: BTreeMap<i32, InternalForceResults>,
}