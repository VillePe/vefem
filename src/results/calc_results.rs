use super::{InternalForceResults, NodeResults};

pub struct CalculationResults {
    pub node_results: NodeResults,
    pub internal_force_results: InternalForceResults,
}