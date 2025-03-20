mod rebar_collection;
mod shear_rebar;
mod rebar_data;
mod tendon_data;
mod calculation_rebar;
pub mod utils;

pub use rebar_collection::RebarCollection;
pub use shear_rebar::ShearRebarGroup;
pub use rebar_data::RebarData;
pub use tendon_data::TendonData;
pub use calculation_rebar::CalculationRebar;
pub use rebar_collection::Side;

use serde::{Deserialize, Serialize};

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

pub trait ReinforcementTrait {
    fn get_yield_strength(&self) -> f64;
    fn get_elastic_modulus(&self) -> f64;
}

impl Default for ElementReinforcement {
    fn default() -> Self {
        Self { main_rebars: Vec::new(), shear_rebars: Vec::new() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RebarDistribution {
    /// Even rebar distribution. To calculate the real positions, the profile values need to be known.
    /// If there is only one rebar, the rebar will be placed with cc_left only and ignoring the cc_right.
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