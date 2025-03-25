use serde::{Deserialize, Serialize};
use vputilslib::geometry2d::{Polygon, VpPoint};

use super::{RebarData, RebarDistribution};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShearRebarGroup {
    pub reinf_data: RebarData,
    pub distribution: RebarDistribution,
    pub shape: Polygon,
}

impl ShearRebarGroup {

    pub fn new_full(reinf_data: RebarData, distribution: RebarDistribution, shape: Polygon) -> Self {
        ShearRebarGroup {
            reinf_data,
            distribution,
            shape,
        }
    }
    pub fn shape_rect(
        bb_width: f64,
        bb_height: f64,
        cc_bot: f64,
        cc_right: f64,
        cc_top: f64,
        cc_left: f64,
    ) -> Polygon {
        Polygon::new(vec![
            VpPoint::new(cc_left, cc_bot),                         // Bot left
            VpPoint::new(bb_width - cc_right, cc_bot),             // Bot right
            VpPoint::new(bb_width - cc_right, bb_height - cc_top), // Top right
            VpPoint::new(cc_left, bb_height - cc_top),             // Top left
            VpPoint::new(cc_left, cc_bot),                         // Bot left (closing the polygon)
        ])
    }
}
