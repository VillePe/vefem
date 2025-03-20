/// Second moment of area calculation methods
use vputilslib::{
    equation_handler::EquationHandler,
    geometry2d::{self, Polygon},
};

use crate::{
    material::Concrete, reinforcement::utils::elastic_centroid, settings::CalculationSettings,
};

use super::PolygonProfile;

/// Calculates the second moment of area with the polygon of the profile. Value in millimeters
/// (mm^4).
/// Returns the absolute value, so the order of points can be clockwise or counter clockwise.
/// For more info see <https://en.wikipedia.org/wiki/Second_moment_of_area>
pub fn smoa_from_polygon(polygon: &Polygon) -> f64 {
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
        sum +=
            (cur_x * next_y - next_x * cur_y) * (cur_y.powi(2) + cur_y * next_y + next_y.powi(2));
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
        crate::material::ConcreteCalcType::Plain => smoa_from_polygon(&profile.polygon),
        crate::material::ConcreteCalcType::WithReinforcement => {
            smoa_with_reinf_internal_uncracked(profile, conc, calc_settings)
        }
        crate::material::ConcreteCalcType::Cracked => {
            smoa_with_reinf_internal_cracked(profile, conc, calc_settings)
        }
    }
}

/// Calculates the second moment of area with uncracked concrete. The value is calculated with
/// the Es/Ec ratio in mind.
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

    cumulative_smoa +=
        smoa_for_polygon + (cog_y - centroid_for_polygon.y).powi(2) * area_for_polygon;

    for r in &concrete.reinforcement.main_rebars {
        for s in r.get_calculation_rebars(profile, &EquationHandler::new()) {
            let es = s.reinf_data.get_elastic_modulus();
            let reduced_area = s.area * (es / ec - 1.0); // Note that the 'hole' is taken into account
                                                         // Can't just multiply the diameter by es/ec because area calculation from diameter is not linear
                                                         // That is why next equation is used to get the diameter: A = pi*d^2/4 => sqrt(4*A / pi)
            let diam_from_red_area = (4.0 * reduced_area / std::f64::consts::PI).sqrt();

            cumulative_smoa +=
                smoa_diameter(diam_from_red_area) + (cog_y - s.y).powi(2) * reduced_area;
        }
    }

    // TODO Needs tests
    cumulative_smoa
}

/// DO NOT USE. NOT YET IMPLEMENTED
/// Calculates the second moment of area with cracked concrete.
fn smoa_with_reinf_internal_cracked(
    profile: &PolygonProfile,
    _conc: &Concrete,
    _calc_settings: &CalculationSettings,
) -> f64 {
    let smoa_for_polygon = smoa_from_polygon(&profile.polygon);

    // Try to find the neutral axis by iterating the neutral axis offset from top and comparing the 
    // elastic area of compression and tension sides. The sum of both sides should be as close to 
    // each other as possible. Notice that the concrete on the tension side is ignored (assumed to
    // be cracked).

    // At the start the absolute value on the compression side should be much lower than
    // the value on the tension side because the neutral axis is almost at the top of the section.
    // The neutral axis is found when the value goes 'over the line' and the sum of the elastic areas
    // is below zero. The value on the compression side is set to negative values and the value on
    // the tension side is positive. 

    // If there are is no reinforcement in the concrete, the neutral axis is the centroid of the polygon.

    // Note that when iterating the neutral axis offset, the polygon needs to be split into two
    // parts to calculate the area on the tension side.

    smoa_for_polygon
}

#[cfg(test)]
mod tests {
    use crate::profile::smoa;
    use crate::{
        material::Concrete,
        profile::Profile,
        reinforcement::{RebarCollection, RebarData, RebarDistribution, ReinforcementData},
        settings::CalculationSettings,
    };

    #[test]
    fn test_smoa_with_reinf() {
        // The example is from book Rakenteiden mekaniikka, Tapio Salmi, Kai Kuula, 2012
        // Notice that the example in the book takes the hole into account in some cases and not others
        let profile = Profile::new_rectangle("name".to_string(), 450.0, 300.0);
        let mut concrete = Concrete {
            elastic_modulus: 25e3,
            char_strength: 1.0,
            ..Default::default()
        };
        concrete
            .reinforcement
            .main_rebars
            .push(RebarCollection::new_bot_full(
                ReinforcementData::Rebar(RebarData::new(500.0, 210e3)),
                RebarDistribution::Even {
                    diam: 15.0,
                    count: 1,
                    cc_left: "300/2-15/2".to_string(),
                    cc_right: "0".to_string(),
                },
                "50-15/2".to_string(),
            ));
        concrete.concrete_calc_type = crate::material::ConcreteCalcType::WithReinforcement;

        let smoa = smoa::smoa_with_reinf(
            &profile.get_polygon_profile(),
            &concrete,
            &CalculationSettings::default(),
        );
        println!("Smoa: {}", smoa);
        println!("Assert: {}", 58.13e12 / 25e3);
        println!("Without rebar: {}", 300e0*450f64.powi(3)/12e0);
        println!("Difference: {:.2}%", 100.0*(1.0 - smoa / (58.13e12 / 25e3)));
        println!("Difference with no rebar: {:.2}%", 100.0*(1.0 - smoa / (300e0*450f64.powi(3)/12e0)));
        assert!((smoa - (58.13e12 / 25e3)).abs() < 1e7);
    }
}
