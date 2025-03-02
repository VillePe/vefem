pub mod profile_custom;
pub mod profile_polygon;
pub mod profile_standard;

pub use profile_custom::ProfileCustom;
pub use profile_polygon::ProfilePolygon;
pub use profile_standard::ProfileStandard;
use vputilslib::geometry2d::{self, rectangle, Polygon, VpPoint};

#[derive(Debug)]
pub enum Profile {
    /// Polygon profile. All the values are calculated from the polygon
    Polygon(ProfilePolygon),
    /// A standard profile from profile library
    Standard(ProfileStandard),
    /// Custom profile most likely created by the user.
    Custom(ProfileCustom),
}
impl Profile {

    /// Creates new profile with the given polygon. Calculates the height and width from
    /// the bounding box of the polygon
    pub fn new(name: String, polygon: Polygon) -> Self {
        // the bounding box
        let bb : geometry2d::Rectangle = rectangle::bounding_box(&polygon).unwrap();
        Profile::Polygon(ProfilePolygon {
            name,
            polygon,
            width: bb.width,
            height: bb.height,
            ..Default::default()
        })
    }
    
    /// Creates new rectangular profile
    pub fn new_rectangle(name: String, height: f64, width: f64) -> Self {
        Profile::Polygon(ProfilePolygon {
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
    
    pub fn get_major_second_mom_of_area(&self) -> f64 {
        match self {
            Profile::Polygon(p) => p.get_major_second_mom_of_area(),
            Profile::Standard(s) => s.get_major_second_mom_of_area(),
            Profile::Custom(c) => c.get_major_second_mom_of_area(),
        }
    }
    
    pub(crate) fn get_area(&self) -> f64 {
        match self {
            Profile::Polygon(p) => p.get_area(),
            Profile::Standard(s) => s.get_area(),
            Profile::Custom(c) => c.get_area(),
        }
    }
}