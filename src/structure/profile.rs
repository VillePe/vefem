#![allow(dead_code)]
use vputilslib::geometry2d::{Polygon, VpPoint};
use vputilslib::geometry2d;

pub struct Profile {
    pub profile_type: ProfileType,
    pub name: String,
    pub height: f64,
    pub width: f64,
    /// Closed polygon for the profile (start and end points are at the same location).
    /// The bottom left point of the bounding box needs to be placed at the origo (0,0). Note that
    /// this doesn't mean that any points need to be at origo, just the bounding box. Points need
    /// to be in counterclockwise order.
    pub polygon: geometry2d::Polygon,
    pub area : f64,
    pub major_mom_of_inertia : f64,
    pub minor_mom_of_inertia : f64,
    pub weight_per_meter : f64,
    pub torsional_constant : f64,
    pub warping_constant : f64,
}

impl Profile {
    pub fn new(name: String, polygon: Polygon) -> Self {
        // the bounding box
        let bb : geometry2d::Rectangle = geometry2d::rectangle::bounding_box(&polygon).unwrap();
        Self {
            profile_type: ProfileType::Polygon,
            name,
            polygon,
            width: bb.width,
            height: bb.height,
            ..Default::default()
        }
    }
    
    pub fn new_rectangle(name: String, height: f64, width: f64) -> Self {
        Self {
            profile_type: ProfileType::Polygon,
            name,
            height,
            width,
            polygon: geometry2d::Polygon::new(vec![
                VpPoint::new(0.0, 0.0),
                VpPoint::new(width, 0.0),
                VpPoint::new(width, height),
                VpPoint::new(0.0, height),
                VpPoint::new(0.0, 0.0),
            ]),
            ..Default::default()
        }
    }
    
    pub fn get_major_mom_of_inertia(&self) -> f64 {
        // Only the polygon type is calculated. Other types have constant values.
        if self.profile_type == ProfileType::Polygon {
            return self.calculate_major_mom_of_inertia()
        }
        self.major_mom_of_inertia
    }
    
    pub fn calculate_major_mom_of_inertia(&self) -> f64 {
        0.0
    }
}

#[derive(PartialEq, Debug)]
pub enum ProfileType {
    /// Polygon profile. All the values are calculated from the polygon
    Polygon,
    /// A standard profile from profile library
    StandardProfile,
    /// Custom profile most likely created by the user.
    Custom
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            profile_type: ProfileType::Polygon,
            name: "".to_string(),
            height: 0.0,
            width: 0.0,
            polygon: Polygon::new(vec![]),
            area : 0.0,
            major_mom_of_inertia : 0.0,
            minor_mom_of_inertia : 0.0,
            weight_per_meter : 0.0,
            torsional_constant : 0.0,
            warping_constant : 0.0,
        }
    }
}
