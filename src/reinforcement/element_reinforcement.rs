use serde::{Deserialize, Serialize};

use super::{RebarData, RebarRow, ShearRebarGroup, TendonData};

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementReinforcement {
    pub main_rebars: Vec<RebarRow>,
    pub shear_rebars: Vec<ShearRebarGroup>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ReinforcementData {
    Rebar(RebarData),
    Tendon(TendonData)
}

pub trait ReinforcementTrait {
    fn get_char_strength(&self) -> f64;
    fn get_elastic_modulus(&self) -> f64;
}
