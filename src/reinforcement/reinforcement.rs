use serde::{Deserialize, Serialize};

use super::{RebarData, RebarCollection, ShearRebarGroup, TendonData};

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementReinforcement {
    pub main_rebars: Vec<RebarCollection>,
    pub shear_rebars: Vec<ShearRebarGroup>,
}


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
    /// Even rebar distribution. To calculate the real positions, the profile values need to be known
    Even{diam: f64, count: isize, cc_left: String, cc_right: String},
    /// Distributed rebar by a distribution string. The first rebar is the first value of the
    /// distribution string. Distribution spaces are separated by a space and multipliers can
    /// be used by using a '*' character (e.g. 30 5*30 60)
    Distributed{diam: f64, distr: String},    
    /// Single rebar at a specific position. The offsets are to the center of the rebar
    Single{diam: f64, off_left: String, off_bot: String},
    /// No real distribution used, only the full area of the reinforcement (not suggested to be used unless testing)
    ByArea{area: f64, mom_of_inertia: f64},
}