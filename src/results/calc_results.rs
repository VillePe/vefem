use std::collections::BTreeMap;

use super::{InternalForceResults, NodeResults};

pub struct CalculationResults {
    pub node_results: NodeResults,
    pub internal_force_results: BTreeMap<i32, InternalForceResults>,
}