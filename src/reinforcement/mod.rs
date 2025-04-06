mod rebar_collection;
mod shear_rebar;
mod rebar_data;
mod tendon_data;
mod calculation_rebar;
mod element_reinforcement;
mod reinforcement_data;
pub mod utils;
pub mod rebar_distribution;

pub use rebar_collection::RebarCollection;
pub use shear_rebar::ShearRebarGroup;
pub use rebar_data::RebarData;
pub use tendon_data::TendonData;
pub use calculation_rebar::CalculationRebar;
pub use rebar_collection::Side;
pub use element_reinforcement::ElementReinforcement;
pub use reinforcement_data::ReinforcementData;
pub use rebar_distribution::RebarDistribution;

pub trait ReinforcementTrait {
    fn get_yield_strength(&self) -> f64;
    fn get_elastic_modulus(&self) -> f64;
}

impl Default for ElementReinforcement {
    fn default() -> Self {
        Self { main_rebars: Vec::new(), shear_rebars: Vec::new() }
    }
}