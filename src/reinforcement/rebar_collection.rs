use serde::{Deserialize, Serialize};

use crate::profile::Profile;

use super::reinforcement::{RebarDistribution, ReinforcementData};

/// A rebar collection for an element. The struct represents a collection of rebars
/// in a concrete section.
#[derive(Debug, Serialize, Deserialize)]
pub struct RebarCollection {
    pub reinf_data: ReinforcementData,
    pub distribution: RebarDistribution,
    pub offset_start: String,
    pub offset_end: String,
    /// The concrete cover for the rebar measured from side property to 'Y' direction, where 'X'
    /// direction is controlled by the side (from left to right). For example, when side is set to
    /// 0, then the concrete cover is measured from bottom to top.
    pub concrete_cover: f64,
    /// Controls where the rebar direction is .
    /// 0 = bottom of the bounding box,
    /// 1 = right,
    /// 2 = top,
    /// 3 = left
    /// 4... = polygons different lines (rotating with how polygon is rotated which can be arbitrary)
    /// 999 = circular
    pub side: i32,
}

impl RebarCollection {

/// Creates a new instance of `RebarCollection` representing a full elements length bottom rebar.
/// 
/// * `reinf_data` - The reinforcement data associated with the rebar.
/// * `distribution` - The distribution of the rebar.
/// * `cc` - The concrete cover for the rebar measured from the side property in the 'Y' direction.
    pub fn new_bot_full(reinf_data: ReinforcementData, distribution: RebarDistribution, cc: f64) -> Self {
        RebarCollection {
            reinf_data,
            distribution,
            offset_start: "0".to_string(),
            offset_end: "L".to_string(),
            concrete_cover: cc,
            side: 0,
        }
    }

    pub fn get_single_rebars(&self, profile: &Profile) {
        todo!()
    }

    fn get_single_rebars_for_polygon(&self, profile: &Profile) {
        todo!()
    }
}

#[allow(dead_code)]
pub enum Side {
    Bottom = 0,
    Right = 1,
    Top = 2,
    Left = 3,
    PolygonStart = 4,
    Circle = 999,
}