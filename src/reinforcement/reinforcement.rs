use serde::{Deserialize, Serialize};

use super::{RebarData, RebarCollection, ShearRebarGroup, TendonData};

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementReinforcement {
    pub main_rebars: Vec<RebarCollection>,
    pub shear_rebars: Vec<ShearRebarGroup>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ReinforcementData {
    Rebar(RebarData),
    Tendon(TendonData)
}

pub trait ReinforcementTrait {
    fn get_yield_strength(&self) -> f64;
    fn get_elastic_modulus(&self) -> f64;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RebarDistribution {
    Real{diam: f64, count: isize},
    Distributed{diam: f64, distr: f64},
    ByArea{area: f64}
}