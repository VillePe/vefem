#![allow(dead_code)]
use vputilslib::geometry2d::{Polygon, VpPoint};
use vputilslib::geometry2d;
use vputilslib::geometry2d::rectangle;

pub struct Profile {
    pub profile_type: ProfileType,
    pub name: String,
    pub height: f64,
    pub width: f64,
    /// Closed polygon for the profile (start and end points are at the same location).
    /// The bottom left point of the bounding box needs to be placed at the origo (0,0). Note that
    /// this doesn't mean that any points need to be at origo, just the bounding box. Points need
    /// to be in counterclockwise order.
    pub polygon: Polygon,
    area : f64,
    major_mom_of_inertia : f64,
    minor_mom_of_inertia : f64,
    pub weight_per_meter : f64,
    pub torsional_constant : f64,
    pub warping_constant : f64,
}

impl Profile {
    pub fn new(name: String, polygon: Polygon) -> Self {
        // the bounding box
        let bb : geometry2d::Rectangle = rectangle::bounding_box(&polygon).unwrap();
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
    
    pub fn get_area(&self) -> f64 {
        vputilslib::geometry2d::calculate_area(&self.polygon)
    }
    
    pub fn get_major_mom_of_inertia(&self) -> f64 {
        // Only the polygon type is calculated. Other types have constant values.
        if self.profile_type == ProfileType::Polygon {
            return self.calculate_major_second_mom_of_area()
        }
        self.major_mom_of_inertia
    }
    
    /// Calculates the moment of inertia with the polygon of the profile.
    /// Returns the absolute value, so the order of points can be clockwise or counter clockwise.
    /// For more info see https://en.wikipedia.org/wiki/Second_moment_of_area
    pub fn calculate_major_second_mom_of_area(&self) -> f64 {
        // Use the centroid to calculate the second moment of area about it
        let centroid = geometry2d::centroid_from_polygon(&self.polygon);
        let mut sum = 0.0;
        let mut next_x : f64;
        let mut next_y : f64;
        for i in 0..self.polygon.points.len() {
            let cur_x = self.polygon.points[i].x-centroid.x;
            let cur_y = self.polygon.points[i].y-centroid.y;
            if i+1 >= self.polygon.points.len() {
                next_x = self.polygon.points[0].x;
                next_y = self.polygon.points[0].y;
            } else {
                next_x = self.polygon.points[i+1].x;
                next_y = self.polygon.points[i+1].y;
            }
            next_x = next_x - centroid.x;
            next_y = next_y - centroid.y;
            sum += (cur_x*next_y - next_x*cur_y)*(cur_x.powi(2) + cur_x*next_x+next_x.powi(2));
        }
        sum.abs() / 12.0
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

#[cfg(test)]
mod tests {
    use vputilslib::geometry2d::VpPoint;
    use crate::structure::profile::Profile;
    use super::*;

    #[test]
    fn major_mom_of_inertia() {
        let p1 = Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0);
        let result = p1.get_major_mom_of_inertia();
        println!("P1 major_mom_of_inertia = {}", result);
        assert!((result-8333333.0) < 1.0);

        let polygon_points2 = vec![
            VpPoint::new(100.0, 0.0),
            VpPoint::new(0.0, 100.0),
            VpPoint::new(100.0, 200.0),
            VpPoint::new(200.0, 100.0),
            VpPoint::new(100.0, 0.0),
        ];
        let p2 = Profile::new("R100x100".to_string(), Polygon::new(polygon_points2));
        let result = p2.get_major_mom_of_inertia();
        println!("P2 major_mom_of_inertia = {}", result);
        assert!((result-33333333.33) < 1.0);

        let polygon_points2_ccw = vec![
            VpPoint::new(100.0, 0.0),
            VpPoint::new(200.0, 100.0),
            VpPoint::new(100.0, 200.0),
            VpPoint::new(0.0, 100.0),
            VpPoint::new(100.0, 0.0),
        ];
        let p2 = Profile::new("R100x100".to_string(), Polygon::new(polygon_points2_ccw));
        let result = p2.get_major_mom_of_inertia();
        println!("P2 ccw major_mom_of_inertia = {}", result);
        assert!((result-33333333.33) < 1.0);
    }
}