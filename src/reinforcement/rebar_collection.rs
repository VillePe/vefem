#![allow(unused_assignments)]
#![allow(unused_variables)]
use std::f64::consts::PI;

use serde::{Deserialize, Serialize};
use vputilslib::{
    equation_handler::EquationHandler,
    geometry2d::polygon::Direction,
};

use crate::profile::PolygonProfile;

use super::{
    RebarDistribution, ReinforcementData,
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
    pub concrete_cover: String,
    /// Controls where the rebar direction is
    pub side: Side,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(tag = "$type")]
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
    // For circular sections. The start angle is in degrees and starts from pointing right and is counter-clockwise.
    // Circular { start_angle: f64 },
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
        cc: String,
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

    pub fn get_row_length(&self, profile: &PolygonProfile) -> f64 {
        let row_length = match &self.side {
            Side::BoundingBox { index } => match index {
                0 => profile.width,
                1 => profile.height,
                2 => profile.width,
                3 => profile.height,
                _ => panic!(),
            },
            Side::Polygon { index } => {
                let (p1, p2) = profile.polygon.get_line_or_last(*index);
                vputilslib::geometry2d::calc_length_between_points(&p1, &p2)
            },
            // Side::Circular { start_angle } => todo!(),
        };
        println!("Row length: {}", row_length);
        row_length
    }

    pub fn get_calculation_rebars(
        &self, 
        profile: &PolygonProfile, 
        equation_handler: &EquationHandler
    ) -> Vec<CalculationRebar> {
        let row_length = self.get_row_length(profile);
        get_calculation_rebars(self, profile, row_length, equation_handler)
    }
}

/// Gathers the single rebars from the rebar collection.
/// 
/// The parser uses an EquationHandler so the strings can contain the 'd' or 'Ø' (alt + 0216 or U+00D8)
/// characters to refer to the diameter value. Parser clones the given equation handler to insert those
/// variables into the equation handler if the 'd' and 'Ø' variables are not already reserved 
/// (in any case the parser does not modify the original)
pub fn get_calculation_rebars(rebar_collection: &RebarCollection, profile: &PolygonProfile, 
    row_length: f64, 
    equation_handler: &EquationHandler
) -> Vec<CalculationRebar> {
    let mut calc_rebars: Vec<CalculationRebar> = Vec::new();
    let mut x = 0.0;
    let mut y = 0.0;
    let offset_start = equation_handler
        .calculate_formula(&rebar_collection.offset_start)
        .unwrap_or(0.0);
    let offset_end = equation_handler
        .calculate_formula(&rebar_collection.offset_end)
        .unwrap_or(0.0);
    match &rebar_collection.distribution {
        RebarDistribution::Even {
                        diam,
                        count,
                        cc_start,
                        cc_end,
            } => {
                let equation_handler = add_diam_to_eq_handler(equation_handler, *diam);
                let cc_left = equation_handler.calculate_formula(&cc_start).unwrap_or(0.0);
                let cc_right = equation_handler.calculate_formula(&cc_end).unwrap_or(0.0);
                let cc_bot = equation_handler.calculate_formula(&rebar_collection.concrete_cover).unwrap_or(0.0);
                // If there is only one, it will be set with left concrete cover
                if *count == 1 {
                    x = cc_left + diam / 2.0;
                    y = cc_bot + diam / 2.0;
                    (x, y) = get_rebar_location_with_side(x, y, &rebar_collection.side, profile);
                    calc_rebars.push(CalculationRebar {
                        area: PI * diam.powi(2) / 4.0,
                        x,
                        y,
                        reinf_data: rebar_collection.reinf_data.clone(),
                        offset_start: offset_start,
                        offset_end: offset_end,
                    });
                } else {
                    let spacing = (row_length - cc_right - cc_left - *diam) / (*count - 1) as f64;
                    for i in 0..*count {
                        x = cc_left + diam / 2.0 + spacing * (i as f64);
                        y = cc_bot + diam / 2.0;
                        (x, y) = get_rebar_location_with_side(x, y, &rebar_collection.side, profile);
                        calc_rebars.push(CalculationRebar {
                            area: PI * diam.powi(2) / 4.0,
                            x,
                            y,
                            reinf_data: rebar_collection.reinf_data.clone(),
                            offset_start: offset_start,
                            offset_end: offset_end,
                        });
                    }
                }            
            }
        RebarDistribution::Distributed { diam, distr } => {
                let mut cumulative_x = 0.0;
                let equation_handler = add_diam_to_eq_handler(equation_handler, *diam);
                let spacings = super::utils::parse_distribution_string(&distr, &equation_handler);
                let cc_bot = equation_handler.calculate_formula(&rebar_collection.concrete_cover).unwrap_or(0.0);
                for i in spacings {
                    cumulative_x += i;
                    y = cc_bot + diam / 2.0;
                    (x, y) = get_rebar_location_with_side(cumulative_x, y, &rebar_collection.side, profile);
                    calc_rebars.push(CalculationRebar {
                        area: PI * diam.powi(2) / 4.0,
                        x,
                        y,
                        reinf_data: rebar_collection.reinf_data.clone(),
                        offset_start: offset_start,
                        offset_end: offset_end,
                    });
                }
            }
        RebarDistribution::ByArea { area , sec_mom_of_area: mom_of_inertia} => {
                // TODO: implement
                todo!()
            }
        RebarDistribution::Single {
                diam,
                off_left,
                off_bot,
            } => {
                let equation_handler = add_diam_to_eq_handler(equation_handler, *diam);
                let x = equation_handler.calculate_formula(&off_left).unwrap_or(0.0);
                let y = equation_handler.calculate_formula(&off_bot).unwrap_or(0.0);
                calc_rebars.push(CalculationRebar {
                    area: PI * diam.powi(2) / 4.0,
                    x,
                    y,
                    reinf_data: rebar_collection.reinf_data.clone(),
                    offset_start: offset_start,
                    offset_end: offset_end,
                });
            },
        RebarDistribution::Spacing { diam, spacing, cc_start, cc_end } => {
            let equation_handler = add_diam_to_eq_handler(equation_handler, *diam);
            let cc_left = equation_handler.calculate_formula(&cc_start).unwrap_or(0.0);
            let cc_right = equation_handler.calculate_formula(&cc_end).unwrap_or(0.0);
            let cc_bot = equation_handler.calculate_formula(&rebar_collection.concrete_cover).unwrap_or(0.0);
            let (spacing, count) = super::utils::get_spacing_and_count(row_length, *diam, cc_right, cc_left, *spacing);
                for i in 0..(count+1) {
                    x = cc_left + diam / 2.0 + spacing * (i as f64);
                    y = cc_bot + diam / 2.0;
                    (x, y) = get_rebar_location_with_side(x, y, &rebar_collection.side, profile);
                    calc_rebars.push(CalculationRebar {
                        area: PI * diam.powi(2) / 4.0,
                        x,
                        y,
                        reinf_data: rebar_collection.reinf_data.clone(),
                        offset_start: offset_start,
                        offset_end: offset_end,
                    });
                }
        },
        }
    calc_rebars
}

/// Adds the diameter of the rebar to equation handlers variables but checks that the variable is 
/// not already set so overriding is not happening (d and Ø are not reserved variables)
/// Returns none if the variable is already set (so the equation handler is not modified)
fn add_diam_to_eq_handler(equation_handler: &EquationHandler, diam: f64) -> EquationHandler {
    let mut equation_handler = equation_handler.clone();
    if !equation_handler.variable_is_set("d") {
        equation_handler.add_variable("d", diam);
    } else if !equation_handler.variable_is_set("Ø") {
        equation_handler.add_variable("Ø", diam);
    }
    equation_handler
}

/// Gets the rebar location based on the side property of the rebar collection. See [RebarCollection::side]
/// ### Arguments
/// * `x` - The x position (center) of the rebar from 0,0 before translating or rotating
/// * `y` - The y position (center) of the rebar from 0,0 before translating or rotating
/// * `side` - The side of the rebar
/// * `index` - The index of the rebar for Side::Polygon.
/// * `profile` - The profile of the element
fn get_rebar_location_with_side(x: f64, y: f64, side: &Side, profile: &PolygonProfile) -> (f64, f64) {
    let mid_w = profile.width / 2.0;
    let mid_h = profile.height / 2.0;
    match side {
        Side::BoundingBox { index } => match index {
            0 => (x, y),
            1 => vputilslib::geometry2d::rotate(mid_w, mid_h, x, y, PI / 2.0),
            2 => vputilslib::geometry2d::rotate(mid_w, mid_h, x, y, PI),
            3 => vputilslib::geometry2d::rotate(mid_w, mid_h, x, y, 3.0 * PI / 2.0),
            _ => panic!(),
        },
        Side::Polygon { index } => {
            let polygon = &profile.polygon;
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
        // Side::Circular { start_angle } => todo!(),
    }
}
