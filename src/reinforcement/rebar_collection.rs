#![allow(unused_assignments)]
#![allow(unused_variables)]
use std::f64::consts::PI;

use serde::{Deserialize, Serialize};
use vputilslib::{
    equation_handler::EquationHandler,
    geometry2d::polygon::Direction,
};

use crate::profile::Profile;

use super::{
    reinforcement::{RebarDistribution, ReinforcementData},
    CalculationRebar,
};

/// A rebar collection for an element. The struct represents a collection of rebars
/// in a concrete section.
#[derive(Debug, Serialize, Deserialize)]
pub struct RebarCollection {
    pub reinf_data: ReinforcementData,
    /// The rebar distribution from left to right in any direction (up is towards middle of the polygon)
    pub distribution: RebarDistribution,
    pub offset_start: String,
    pub offset_end: String,
    /// The concrete cover for the rebar measured from side property to 'Y' direction, where 'X'
    /// direction is controlled by the side (from left to right). For example, when side is set to
    /// 0, then the concrete cover is measured from bottom to top.
    pub concrete_cover: f64,
    /// Controls where the rebar direction is
    pub side: Side,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Side {
    /// The index argument controls where the rebar is placed.
    /// - 0 = bottom of the bounding box,
    /// - 1 = right,
    /// - 2 = top,
    /// - 3 = left
    BoundingBox { index: i32 },
    /// For polygon type, the index argument is by what line of the polygon the rebars are placed.
    /// For this the orientation should be known and line count of the polygon needs to be known or
    /// the program will panic if the index is out of the range of the lines.
    Polygon { index: i32 },
    /// For circular sections. The start angle is in degrees and starts from pointing right and is counter-clockwise.
    Circular { start_angle: f64 },
}

impl RebarCollection {
    /// Creates a new instance of `RebarCollection` representing a full elements length bottom rebar.
    ///
    /// * `reinf_data` - The reinforcement data associated with the rebar.
    /// * `distribution` - The distribution of the rebar.
    /// * `cc` - The concrete cover for the rebar measured from the side property in the 'Y' direction.
    pub fn new_bot_full(
        reinf_data: ReinforcementData,
        distribution: RebarDistribution,
        cc: f64,
    ) -> Self {
        RebarCollection {
            reinf_data,
            distribution,
            offset_start: "0".to_string(),
            offset_end: "L".to_string(),
            concrete_cover: cc,
            side: Side::BoundingBox { index: (0) },
        }
    }

    pub fn get_single_rebars(
        &self,
        profile: &Profile,
        equation_handler: &EquationHandler,
    ) -> Vec<CalculationRebar> {
        let mut single_rebars: Vec<CalculationRebar> = Vec::new();
        let offset_start = equation_handler
            .calculate_formula(&self.offset_start)
            .unwrap_or(0.0);
        let offset_end = equation_handler
            .calculate_formula(&self.offset_end)
            .unwrap_or(0.0);
        let mut x = 0.0;
        let mut y = 0.0;
        let row_length = match &self.side {
            Side::BoundingBox { index } => match index {
                0 => profile.get_width(),
                1 => profile.get_height(),
                2 => profile.get_width(),
                3 => profile.get_height(),
                _ => panic!(),
            },
            Side::Polygon { index } => match profile {
                Profile::PolygonProfile(polygon_profile) => {
                    let (p1, p2) = polygon_profile.polygon.get_line_or_last(*index);
                    vputilslib::geometry2d::calc_length_between_points(&p1, &p2)
                }
                _ => profile.get_width(),
            },
            Side::Circular { start_angle } => todo!(),
        };
        println!("Row length: {}", row_length);

        match &self.distribution {
            RebarDistribution::Even {
                diam,
                count,
                cc_left,
                cc_right,
            } => {
                let cc_left = equation_handler.calculate_formula(&cc_left).unwrap_or(0.0);
                let cc_right = equation_handler.calculate_formula(&cc_right).unwrap_or(0.0);
                // If there is only one, it will be set with left concrete cover
                if *count == 1 {
                    x = cc_left + diam / 2.0;
                    y = self.concrete_cover + diam / 2.0;
                    (x, y) = get_rebar_location_with_side(x, y, &self.side, profile);
                    single_rebars.push(CalculationRebar {
                        area: PI * diam.powi(2) / 4.0,
                        x,
                        y,
                        reinf_data: self.reinf_data.clone(),
                        offset_start: offset_start,
                        offset_end: offset_end,
                    });
                };
                let spacing = (row_length - cc_right - cc_left - *diam) / (*count - 1) as f64;
                for i in 0..*count {
                    x = cc_left + diam / 2.0 + spacing * (i as f64);
                    y = self.concrete_cover + diam / 2.0;
                    (x, y) = get_rebar_location_with_side(x, y, &self.side, profile);
                    single_rebars.push(CalculationRebar {
                        area: PI * diam.powi(2) / 4.0,
                        x,
                        y,
                        reinf_data: self.reinf_data.clone(),
                        offset_start: offset_start,
                        offset_end: offset_end,
                    });
                }
            }
            RebarDistribution::Distributed { diam, distr } => {
                // TODO: implement
                todo!()
            }
            RebarDistribution::ByArea { area } => {
                // TODO: implement
                todo!()
            }
            RebarDistribution::Single {
                diam,
                off_left,
                off_bot,
            } => todo!(),
        }
        single_rebars
    }
}

/// Gets the rebar location based on the side property of the rebar collection. See [RebarCollection::side]
/// ### Arguments
/// * `x` - The x position (center) of the rebar from 0,0 before translating or rotating
/// * `y` - The y position (center) of the rebar from 0,0 before translating or rotating
/// * `side` - The side of the rebar
/// * `index` - The index of the rebar for Side::Polygon.
/// * `profile` - The profile of the element
fn get_rebar_location_with_side(x: f64, y: f64, side: &Side, profile: &Profile) -> (f64, f64) {
    let mid_w = profile.get_width() / 2.0;
    let mid_h = profile.get_height() / 2.0;
    match side {
        Side::BoundingBox { index } => match index {
            0 => (x, y),
            1 => vputilslib::geometry2d::rotate(mid_w, mid_h, x, y, PI / 2.0),
            2 => vputilslib::geometry2d::rotate(mid_w, mid_h, x, y, PI),
            3 => vputilslib::geometry2d::rotate(mid_w, mid_h, x, y, 3.0 * PI / 2.0),
            _ => panic!(),
        },
        Side::Polygon { index } => {
            match profile {
                Profile::PolygonProfile(polygon_profile) => {
                    let polygon = &polygon_profile.polygon;
                    let (p1, p2) = polygon.get_line_or_last(*index);
                    let rotate_angle = vputilslib::geometry2d::get_angle_from_points(p1, p2);
                    let mut yoffset = 0.0;
                    // If the polygon is clockwise, the y value needs to be inversed
                    if polygon.get_direction() == Direction::Clockwise { yoffset = -y * 2.0; }
                    // Rotate the point around (0,0)
                    let (mut rx, mut ry) =
                        vputilslib::geometry2d::rotate(0.0, 0.0, x, y+yoffset, rotate_angle);
                    // Move the rotated point with move vector from (0,0) to p1
                    rx += p1.x;
                    ry += p1.y;
                    (rx, ry)
                }
                _ => (0.0, 0.0),
            }
        }
        Side::Circular { start_angle } => todo!(),
    }
}

#[cfg(test)]
mod reinf_tests {
    use std::result;

    use vputilslib::geometry2d::{Polygon, VpPoint};

    use crate::reinforcement::RebarData;

    use super::*;

    #[test]
    fn test_get_single_rebars_bbox() {
        let side = Side::BoundingBox { index: 0 };
        let profile = Profile::new_rectangle("name".to_string(), 480.0, 280.0);
        let reinf_data = ReinforcementData::Rebar(RebarData {
            yield_strength: 500.0,
            elastic_modulus: 200e3,
        });
        let offset_start = "0".to_string();
        let offset_end = "L".to_string();
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("L", 4000.0);

        let rebar_coll1: RebarCollection = RebarCollection {
            reinf_data,
            offset_start,
            offset_end,
            concrete_cover: 30.0,
            side,
            distribution: RebarDistribution::Even {
                diam: 16.0,
                count: 4,
                cc_left: "30".to_string(),
                cc_right: "30".to_string(),
            },
        };
        let result1 = rebar_coll1.get_single_rebars(&profile, &equation_handler);
        for rebar in &result1 {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!(result1[0].x == 38.0);
        assert!(result1[1].x == 106.0);
        assert!(result1[2].x == 174.0);
        assert!(result1[3].x == 242.0);
    }

    #[test]
    fn test_get_single_rebars_polygon() {
        let side = Side::Polygon { index: 0 };
        let profile = Profile::new_rectangle("name".to_string(), 480.0, 280.0);
        let reinf_data = ReinforcementData::Rebar(RebarData {
            yield_strength: 500.0,
            elastic_modulus: 200e3,
        });
        let offset_start = "0".to_string();
        let offset_end = "L".to_string();
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("L", 4000.0);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side,
            distribution: RebarDistribution::Even {
                diam: 16.0,
                count: 4,
                cc_left: "30".to_string(),
                cc_right: "30".to_string(),
            },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!(result[0].x == 38.0);
        assert!(result[1].x == 106.0);
        assert!(result[2].x == 174.0);
        assert!(result[3].x == 242.0);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side: Side::Polygon { index: 1 },
            distribution: RebarDistribution::Even {
                diam: 16.0,
                count: 4,
                cc_left: "30".to_string(),
                cc_right: "30".to_string(),
            },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {:.2}, Y: {:.2}", rebar.x, rebar.y);
        }
        assert!((result[0].y - 38.00).abs() < 0.01);
        assert!((result[1].y - 172.67).abs() < 0.01);
        assert!((result[2].y - 307.33).abs() < 0.01);
        assert!((result[3].y - 442.00).abs() < 0.01);
    }

    #[test]
    fn test_get_single_rebars_polygon_clockwise() {
        let profile = Profile::new("name".to_string(), Polygon::new(vec![
            VpPoint::new(280.0, 0.0),
            VpPoint::new(0.0, 0.0),
            VpPoint::new(0.0, 480.0),
            VpPoint::new(280.0, 480.0),
            VpPoint::new(280.0, 0.0),
        ]));
        let reinf_data = ReinforcementData::Rebar(RebarData {
            yield_strength: 500.0,
            elastic_modulus: 200e3,
        });
        let offset_start = "0".to_string();
        let offset_end = "L".to_string();
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("L", 4000.0);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side: Side::Polygon { index: 1 },
            distribution: RebarDistribution::Even {
                diam: 16.0,
                count: 4,
                cc_left: "30".to_string(),
                cc_right: "30".to_string(),
            },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!((result[0].y - 38.00).abs() < 0.01);
        assert!((result[1].y - 172.67).abs() < 0.01);
        assert!((result[2].y - 307.33).abs() < 0.01);
        assert!((result[3].y - 442.00).abs() < 0.01);
    }
}
