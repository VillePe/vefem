use vputilslib::{equation_handler::EquationHandler, geometry2d::{self, Polygon}};

use crate::{material::Concrete, reinforcement::utils::elastic_centroid, settings::CalculationSettings};

use super::PolygonProfile;

/// Calculates the second moment of area with the polygon of the profile. Value in millimeters
    /// (mm^4).
    /// Returns the absolute value, so the order of points can be clockwise or counter clockwise.
    /// For more info see <https://en.wikipedia.org/wiki/Second_moment_of_area>
    pub fn smoa_from_polygon(polygon: &Polygon,) -> f64 {
        // Use the centroid to calculate the second moment of area about it
        let centroid = geometry2d::centroid_from_polygon(polygon);
        let mut sum = 0.0;
        let mut next_x: f64;
        let mut next_y: f64;
        for i in 0..polygon.points.len() {
            let cur_x = polygon.points[i].x - centroid.x;
            let cur_y = polygon.points[i].y - centroid.y;
            if i + 1 >= polygon.points.len() {
                next_x = polygon.points[0].x;
                next_y = polygon.points[0].y;
            } else {
                next_x = polygon.points[i + 1].x;
                next_y = polygon.points[i + 1].y;
            }
            next_x = next_x - centroid.x;
            next_y = next_y - centroid.y;
            sum += (cur_x * next_y - next_x * cur_y)
                * (cur_y.powi(2) + cur_y * next_y + next_y.powi(2));
        }
        sum.abs() / 12.0
    }

    pub fn smoa_radius(radius: f64) -> f64 {
        std::f64::consts::PI * radius.powi(4) / 4.0
    }

    pub fn smoa_diameter(diameter: f64) -> f64 {
        smoa_radius(diameter / 2.0)
    }

    pub fn smoa_with_reinf(
        profile: &PolygonProfile,
        conc: &Concrete,
        calc_settings: &CalculationSettings,
    ) -> f64 {
        match conc.concrete_calc_type {
            crate::material::ConcreteCalcType::Plain => {
                smoa_from_polygon(&profile.polygon)
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
        concrete: &Concrete,
        calc_settings: &CalculationSettings,
    ) -> f64 {
        let mut cumulative_smoa = 0.0;
        let smoa_for_polygon = smoa_from_polygon(&profile.polygon);
        let area_for_polygon = geometry2d::calculate_area(&profile.polygon);
        let centroid_for_polygon = geometry2d::centroid_from_polygon(&profile.polygon);
        let (_cog_x, cog_y) = elastic_centroid(profile, &concrete, calc_settings);
        let ec = concrete.elastic_modulus;

        cumulative_smoa += smoa_for_polygon + (cog_y-centroid_for_polygon.y).powi(2) * area_for_polygon;

        for r in &concrete.reinforcement.main_rebars {
            for s in r.get_calculation_rebars(profile, &EquationHandler::new()) { 
                let es = s.reinf_data.get_elastic_modulus();
                let reduced_area = s.area * (es / ec - 1.0); // Note that the 'hole' is taken into account
                // A = pi*d^2/4 => sqrt(4*A / pi)
                let diam_from_red_area = (4.0 * reduced_area / std::f64::consts::PI).sqrt();

                cumulative_smoa +=  smoa_diameter(diam_from_red_area) + (cog_y - s.y).powi(2) * reduced_area;
            }
        }

        // TODO Needs tests
        cumulative_smoa
    }

    /// Calculates the second moment of area with cracked concrete.
    fn smoa_with_reinf_internal_cracked(
        profile: &PolygonProfile,
        conc: &Concrete,
        calc_settings: &CalculationSettings,
    ) -> f64 {
        let smoa_for_polygon = smoa_from_polygon(&profile.polygon);

        smoa_for_polygon
    }