#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use vputilslib::equation_handler::EquationHandler;
use vputilslib::geometry2d;
use vputilslib::geometry2d::rectangle;
use vputilslib::geometry2d::{Polygon, VpPoint};

use crate::material::{Concrete, MaterialData};
use crate::settings::CalculationSettings;

use super::{second_moment_of_area, Profile};

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonProfile {
    /// Name for the profile. If profile type is set to StandardProfile, the values are read from profile
    /// database with the name
    pub name: String,
    /// The height of the bounding box of the profile
    pub height: f64,
    /// The width of the bounding box of the profile
    pub width: f64,
    /// Closed polygon for the profile (start and end points are at the same location).
    /// The bottom left point of the bounding box needs to be placed at the origo (0,0). Note that
    /// this doesn't mean that any points need to be at origo, just the bounding box. Points need
    /// to be in counterclockwise order.
    ///
    /// For example
    /// ```no_run
    /// use vputilslib::geometry2d::{Polygon, VpPoint};
    ///  let polygon = Polygon::new(vec![
    ///     VpPoint::new(0.0, 0.0),
    ///     VpPoint::new(100.0, 0.0),
    ///     VpPoint::new(100.0, 200.0),
    ///     VpPoint::new(0.0, 200.0),
    ///     VpPoint::new(0.0, 0.0),
    ///  ]);
    /// ```
    pub polygon: Polygon,
}

impl PolygonProfile {
    /// Creates new profile with the given polygon. Calculates the height and width from
    /// the bounding box of the polygon
    pub fn new(name: String, polygon: Polygon) -> Self {
        // the bounding box
        let bb: geometry2d::Rectangle = rectangle::bounding_box(&polygon).unwrap();
        Self {
            name,
            polygon,
            width: bb.width,
            height: bb.height,
            ..Default::default()
        }
    }

    /// Creates new rectangular profile
    pub fn new_rectangle(name: String, height: f64, width: f64) -> Self {
        Self {
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
        }
    }

    /// Gets the area of the profile in square millimeters (mm²)
    pub fn get_area(&self, material: &MaterialData, _calc_settings: &CalculationSettings) -> f64 {
        // Only the polygon type is calculated. Other types have constant values.
        match material {
            MaterialData::Concrete(concrete) => {
                self.get_area_conc_section(concrete, _calc_settings)
            }
            _ => geometry2d::calculate_area(&self.polygon),
        }
    }

    /// Gets the area of the profile in square millimeters (mm²)
    pub fn get_area_conc_section(&self, concrete: &Concrete, _calc_settings: &CalculationSettings) -> f64 {
        // Only the polygon type is calculated. Other types have constant values.
        let mut area = geometry2d::calculate_area(&self.polygon);
        for r in &concrete.reinforcement.main_rebars {
            for s in r.get_calculation_rebars(self, &EquationHandler::new()) {
                let reduced_area = s.area * (s.reinf_data.get_elastic_modulus() / concrete.elastic_modulus - 1.0); // Note that the 'hole' is taken into account
                area += reduced_area;                        
            }
        }
        area
    }

    /// Calculates the second moment of area with the polygon of the profile. Value in millimeters
    /// (mm^4).
    /// Returns the absolute value, so the order of points can be clockwise or counter clockwise.
    /// For more info see <https://en.wikipedia.org/wiki/Second_moment_of_area>
    pub fn get_major_second_mom_of_area(
        &self,
        material: &MaterialData,
        calc_settings: &CalculationSettings,
    ) -> f64 {
        // Only the polygon type is calculated. Other types have constant values.
        match material {
            MaterialData::Concrete(concrete) => {
                // TODO
                second_moment_of_area::smoa_with_reinf(self, concrete, calc_settings)
            }
            _ => second_moment_of_area::smoa_from_polygon(&self.polygon),
        }
    }

    
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum ProfileType {
    /// Polygon profile. All the values are calculated from the polygon
    Polygon,
    /// A standard profile from profile library
    StandardProfile,
    /// Custom profile most likely created by the user.
    Custom,
}

impl Default for PolygonProfile {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            height: 0.0,
            width: 0.0,
            polygon: Polygon::new(vec![]),
        }
    }
}

impl TryFrom<Profile> for PolygonProfile {
    type Error = &'static str;
    fn try_from(value: Profile) -> Result<Self, Self::Error> {
        match value {
            Profile::PolygonProfile(p) => Result::Ok(p),
            _ => Result::Err("Wrong profile type!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::profile::Profile;
    use vputilslib::geometry2d::VpPoint;

    use super::*;

    #[test]
    fn major_second_mom_of_area() {
        let p1: Profile = Profile::PolygonProfile(PolygonProfile::new_rectangle(
            "R100x100".to_string(),
            100.0,
            100.0,
        ));
        let result = p1.get_major_second_mom_of_area(
            &MaterialData::default(),
            &CalculationSettings::default(),
        );
        println!("P1 major_second_mom_of_area = {}", result);
        assert!((result - 8333333.0).abs() < 1.0);

        let polygon_points2 = vec![
            VpPoint::new(100.0, 0.0),
            VpPoint::new(0.0, 100.0),
            VpPoint::new(100.0, 200.0),
            VpPoint::new(200.0, 100.0),
            VpPoint::new(100.0, 0.0),
        ];
        let p2 = Profile::new("R100x100".to_string(), Polygon::new(polygon_points2));
        let result = p2.get_major_second_mom_of_area(
            &MaterialData::default(),
            &CalculationSettings::default(),
        );
        println!("P2 major_second_mom_of_area = {}", result);
        assert!((result - 33333333.33).abs() < 1.0);

        let polygon_points2_ccw = vec![
            VpPoint::new(100.0, 0.0),
            VpPoint::new(200.0, 100.0),
            VpPoint::new(100.0, 200.0),
            VpPoint::new(0.0, 100.0),
            VpPoint::new(100.0, 0.0),
        ];
        let p2 = Profile::new("R100x100".to_string(), Polygon::new(polygon_points2_ccw));
        let result = p2.get_major_second_mom_of_area(
            &MaterialData::default(),
            &CalculationSettings::default(),
        );
        println!("P2 ccw major_second_mom_of_area = {}", result);
        assert!((result - 33333333.33).abs() < 1.0);

        let polygon_points3 = vec![
            VpPoint::new(0.0, 0.0),
            VpPoint::new(0.0, 200.0),
            VpPoint::new(100.0, 200.0),
            VpPoint::new(100.0, 0.0),
            VpPoint::new(0.0, 0.0),
        ];
        let p3 = Profile::new("R200x100".to_string(), Polygon::new(polygon_points3));
        let result = p3.get_major_second_mom_of_area(
            &MaterialData::default(),
            &CalculationSettings::default(),
        );
        println!("P3 ccw major_second_mom_of_area = {}", result);
        assert!((result - 66666666.666666666).abs() < 1.0);

        let p4 = Profile::new_rectangle("R200x100".to_string(), 200.0, 100.0);
        let result = p4.get_major_second_mom_of_area(
            &MaterialData::default(),
            &CalculationSettings::default(),
        );
        println!("P4 major_second_mom_of_area = {}", result);
        assert!((result - 66666666.666666666).abs() < 1.0);
    }
}
