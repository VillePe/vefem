use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CalculationSettings {
    pub calc_split_interval: CalcSplitInterval,
    pub calc_threaded: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "$type", content = "value")]
pub enum CalcSplitInterval {
    /// Splits the calculation into intervals of a fixed length.
    Absolute(f64),
    /// Splits the calculation into intervals relative to the length of the element.
    Relative(f64),
}

impl Default for CalculationSettings {
    fn default() -> Self {
        Self {
            calc_split_interval: CalcSplitInterval::Relative(0.01),
            calc_threaded: true,
        }
    }
}