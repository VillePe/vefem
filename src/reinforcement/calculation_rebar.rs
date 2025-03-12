use serde::{Deserialize, Serialize};

use super::reinforcement::ReinforcementData;

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculationRebar {
    /// The rebar data
    pub reinf_data: ReinforcementData,
    /// Area of the rebar. For rebar this should be calculated from the diameter and for a tendon
    /// this should be the nominal area of the tendon (can't be directly calculated from the diameter)
    pub area: f64,
    /// The X coordinate in concrete sections coordinate system to the middle of the rebar
    pub x: f64,
    /// The Y coordinate in concrete sections coordinate system to the middle of the rebar
    pub y: f64,
    /// The offset from the start of the element to the start of the rebar
    pub offset_start: f64,
    /// The offset from the start of the element to the end of the rebar
    pub offset_end: f64,
}