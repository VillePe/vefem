#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomProfile {
    /// Name for the profile. If profile type is set to StandardProfile, the values are read from profile
    /// database with the name
    pub name: String,
    /// The height of the bounding box of the profile
    pub height: f64,
    /// The width of the bounding box of the profile
    pub width: f64,
    /// Custom area for StandardProfile or Custom profile types
    pub custom_area: f64,
    /// Custom major second moment of area for StandardProfile or Custom profile types
    pub custom_major_sec_mom_of_area: f64,
    /// Custom minor second moment of area for StandardProfile or Custom profile types
    pub custom_minor_sec_mom_of_area: f64,
    /// Custom weight for StandardProfile or Custom profile types
    pub custom_weight_per_meter: f64,
    /// Custom torsional constant for StandardProfile or Custom profile types
    pub custom_torsional_constant: f64,
    /// Custom warping constant for StandardProfile or Custom profile types
    pub custom_warping_constant: f64,
}

impl CustomProfile {
    
    /// Gets the area of the profile in square millimeters (mm²)
    pub fn get_area(&self) -> f64 {
        self.custom_area
    }

    /// Calculates the second moment of area with the polygon of the profile. Value in millimeters 
    /// (mm^4).
    /// Returns the absolute value, so the order of points can be clockwise or counter clockwise.
    /// For more info see <https://en.wikipedia.org/wiki/Second_moment_of_area>
    pub fn get_major_second_mom_of_area(&self) -> f64 {
        self.custom_major_sec_mom_of_area
    }
}

impl Default for CustomProfile {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            height: 0.0,
            width: 0.0,
            custom_area: 0.0,
            custom_major_sec_mom_of_area: 0.0,
            custom_minor_sec_mom_of_area: 0.0,
            custom_weight_per_meter: 0.0,
            custom_torsional_constant: 0.0,
            custom_warping_constant: 0.0,
        }
    }
}