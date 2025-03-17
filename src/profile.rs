pub mod profile_custom;
pub mod profile_polygon;
pub mod profile_standard;

pub use profile_custom::CustomProfile;
pub use profile_polygon::PolygonProfile;
pub use profile_standard::StandardProfile;
use serde::{Deserialize, Serialize};
use vputilslib::geometry2d::{self, rectangle, Polygon, VpPoint};

use crate::material::MaterialData;

#[derive(Debug, Serialize, Deserialize)]
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

    /// Creates new profile with the given polygon. Calculates the height and width from
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
    
    /// Creates new rectangular profile
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
    
    pub fn get_major_second_mom_of_area(&self, material: &MaterialData) -> f64 {
        match self {
            Profile::PolygonProfile(p) => p.get_major_second_mom_of_area(material),
            Profile::StandardProfile(s) => s.get_major_second_mom_of_area(),
            Profile::CustomProfile(c) => c.get_major_second_mom_of_area(),
        }
    }
    
    pub fn get_area(&self) -> f64 {
        match self {
            Profile::PolygonProfile(p) => p.get_area(),
            Profile::StandardProfile(s) => s.get_area(),
            Profile::CustomProfile(c) => c.get_area(),
        }
    }

    pub fn get_width(&self) -> f64 {
        match self {
            Profile::PolygonProfile(p) => p.width,
            Profile::StandardProfile(s) => s.width,
            Profile::CustomProfile(c) => c.width,
        }
    }

    pub fn get_height(&self) -> f64 {
        match self {
            Profile::PolygonProfile(p) => p.height,
            Profile::StandardProfile(s) => s.height,
            Profile::CustomProfile(c) => c.height,
        }
    }
}