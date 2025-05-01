pub mod profile_custom;
pub mod profile_polygon;
pub mod profile_standard;
pub mod smoa;
pub mod area;

pub use profile_custom::CustomProfile;
pub use profile_polygon::PolygonProfile;
pub use profile_standard::StandardProfile;
use serde::{Deserialize, Serialize};
use vputilslib::geometry2d::{self, rectangle, Polygon, VpPoint};

use crate::{material::MaterialData, settings::CalculationSettings};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "$type", content = "data")]
pub enum Profile {
    /// Polygon profile. All the values are calculated from the polygon
    PolygonProfile(PolygonProfile),
    /// A standard profile from profile library
    StandardProfile(StandardProfile),
    /// Custom profile most likely created by the user.
    CustomProfile(CustomProfile),
}
impl Profile {

    /// Creates new (Polygon) profile with the given polygon. Calculates the height and width from
    /// the bounding box of the polygon
    pub fn new(name: String, polygon: Polygon) -> Self {
        // the bounding box
        let bb : geometry2d::Rectangle = rectangle::bounding_box(&polygon).unwrap();
        Profile::PolygonProfile(PolygonProfile {
            name,
            polygon,
            width: bb.width,
            height: bb.height,
            ..Default::default()
        })
    }
    
    /// Creates new rectangular (Polygon) profile
    pub fn new_rectangle(name: String, height: f64, width: f64) -> Self {
        Profile::PolygonProfile(PolygonProfile {
            name,
            height,
            width,
            polygon: Polygon::new(vec![
                VpPoint::new(0.0, 0.0),
                VpPoint::new(width, 0.0),
                VpPoint::new(width, height),
                VpPoint::new(0.0, height),
                VpPoint::new(0.0, 0.0),
            ]),
            ..Default::default()
        })
    }
    
    /// Gets the second moment of area about the major axis for given profile. For polygon profile
    /// type, the second moment of area is calculated with the polygon of the profile. If the material
    /// is concrete and reinforcement is provided, it is taken into account in polygon type profiles.
    /// 
    /// For standard and custom profiles, the values are taken from the profile properties. Note that
    /// for these types of profiles, reinforcement does not have any effect.
    pub fn get_major_second_mom_of_area(&self, material: &MaterialData, calc_settings: &CalculationSettings) -> f64 {
        match self {
            Profile::PolygonProfile(p) => p.get_major_second_mom_of_area(material, calc_settings),
            Profile::StandardProfile(s) => s.get_major_second_mom_of_area(),
            Profile::CustomProfile(c) => c.get_major_second_mom_of_area(),
        }
    }
    
    /// Gets the area of the profile in square millimeters (mm²)
    pub fn get_area(&self, material: &MaterialData, calc_settings: &CalculationSettings) -> f64 {
        match self {
            Profile::PolygonProfile(p) => p.get_area(material, calc_settings),
            Profile::StandardProfile(s) => s.get_area(),
            Profile::CustomProfile(c) => c.get_area(),
        }
    }

    /// Gets the width of the profile
    pub fn get_width(&self) -> f64 {
        match self {
            Profile::PolygonProfile(p) => p.width,
            Profile::StandardProfile(s) => s.width,
            Profile::CustomProfile(c) => c.width,
        }
    }

    /// Gets the height of the profile
    pub fn get_height(&self) -> f64 {
        match self {
            Profile::PolygonProfile(p) => p.height,
            Profile::StandardProfile(s) => s.height,
            Profile::CustomProfile(c) => c.height,
        }
    }

    /// Gets the polygon profile from the enum. Panics if the profile is not a polygon profile
    pub fn get_polygon_profile (&self) -> &PolygonProfile {
        match self {
            Profile::PolygonProfile(p) => p,
            _ => panic!(),
        }
    }
}