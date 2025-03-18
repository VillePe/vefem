use vputilslib::geometry2d;

use crate::{material::Concrete, settings::CalculationSettings};

use super::PolygonProfile;

/// Calculates the second moment of area with the polygon of the profile. Value in millimeters
    /// (mm^4).
    /// Returns the absolute value, so the order of points can be clockwise or counter clockwise.
    /// For more info see <https://en.wikipedia.org/wiki/Second_moment_of_area>
    pub fn smoa_from_polygon(profile: &PolygonProfile,) -> f64 {
        // Use the centroid to calculate the second moment of area about it
        let centroid = geometry2d::centroid_from_polygon(&profile.polygon);
        let mut sum = 0.0;
        let mut next_x: f64;
        let mut next_y: f64;
        for i in 0..profile.polygon.points.len() {
            let cur_x = profile.polygon.points[i].x - centroid.x;
            let cur_y = profile.polygon.points[i].y - centroid.y;
            if i + 1 >= profile.polygon.points.len() {
                next_x = profile.polygon.points[0].x;
                next_y = profile.polygon.points[0].y;
            } else {
                next_x = profile.polygon.points[i + 1].x;
                next_y = profile.polygon.points[i + 1].y;
            }
            next_x = next_x - centroid.x;
            next_y = next_y - centroid.y;
            sum += (cur_x * next_y - next_x * cur_y)
                * (cur_y.powi(2) + cur_y * next_y + next_y.powi(2));
        }
        sum.abs() / 12.0
    }

    pub fn smoa_with_reinf(
        profile: &PolygonProfile,
        conc: &Concrete,
        calc_settings: &CalculationSettings,
    ) -> f64 {
        match conc.concrete_calc_type {
            crate::material::ConcreteCalcType::Plain => {
                smoa_from_polygon(profile)
            }
            crate::material::ConcreteCalcType::WithReinforcement => {
                smoa_with_reinf_internal_uncracked(profile, conc, calc_settings)
            }
            crate::material::ConcreteCalcType::Cracked => {
                smoa_with_reinf_internal_cracked(profile, conc, calc_settings)
            }
        }
    }

    /// Calculates the second moment of area with uncracked concrete.
    fn smoa_with_reinf_internal_uncracked(
        profile: &PolygonProfile,
        conc: &Concrete,
        calc_settings: &CalculationSettings,
    ) -> f64 {
        let smoa_for_polygon = smoa_from_polygon(profile);

        smoa_for_polygon
    }

    /// Calculates the second moment of area with cracked concrete.
    fn smoa_with_reinf_internal_cracked(
        profile: &PolygonProfile,
        conc: &Concrete,
        calc_settings: &CalculationSettings,
    ) -> f64 {
        let smoa_for_polygon = smoa_from_polygon(profile);

        smoa_for_polygon
    }