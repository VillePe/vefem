use serde::{Deserialize, Serialize};

use super::{RebarData, ReinforcementTrait, TendonData};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(tag = "$type", content = "data")]
pub enum ReinforcementData {
    Rebar(RebarData),
    Tendon(TendonData)
}

impl ReinforcementData {
    pub fn get_yield_strength(&self) -> f64 {
        match self {
            ReinforcementData::Rebar(r) => r.get_yield_strength(),
            ReinforcementData::Tendon(t) => t.get_yield_strength(),
        }
    }

    pub fn get_elastic_modulus(&self) -> f64 {
        match self {
            ReinforcementData::Rebar(r) => r.get_elastic_modulus(),
            ReinforcementData::Tendon(t) => t.get_elastic_modulus(),
        }
    }
}