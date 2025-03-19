#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::Profile;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomProfile {
    /// Name for the profile. If profile type is set to CustomProfile, the values are read from profile
    /// database with the name
    pub name: String,
    /// The height of the bounding box of the profile
    pub height: f64,
    /// The width of the bounding box of the profile
    pub width: f64,
    /// Custom area for CustomProfile or Custom profile types
    pub custom_area: f64,
    /// Custom major second moment of area for CustomProfile or Custom profile types
    pub custom_major_sec_mom_of_area: f64,
    /// Custom minor second moment of area for CustomProfile or Custom profile types
    pub custom_minor_sec_mom_of_area: f64,
    /// Custom weight for CustomProfile or Custom profile types
    pub custom_weight_per_meter: f64,
    /// Custom torsional constant for CustomProfile or Custom profile types
    pub custom_torsional_constant: f64,
    /// Custom warping constant for CustomProfile or Custom profile types
    pub custom_warping_constant: f64,
    /// Custom X-value of the center of gravity for CustomProfile or Custom profile types
    pub center_of_gravity_x: f64,
    /// Custom Y-value of thecenter of gravity for CustomProfile or Custom profile types
    pub center_of_gravity_y: f64,
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
            center_of_gravity_x: 0.0,
            center_of_gravity_y: 0.0
        }
    }
}

impl TryFrom<Profile> for CustomProfile {
    type Error = &'static str;
    fn try_from(value: Profile) -> Result<Self, Self::Error> {
        match value {
            Profile::CustomProfile(p) => Result::Ok(p),
            _ => Result::Err("Wrong profile type!"),
        }
    }
}