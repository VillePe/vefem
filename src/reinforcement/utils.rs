use vputilslib::{equation_handler::EquationHandler, geometry2d};

use crate::{material::Concrete, profile::PolygonProfile, settings::CalculationSettings};

    /// Parses a distribution string into a vector of spacing values. The string is formatted with
    /// space separated values and can contain values with '*' character (e.g. 5*60). The '*' character
    /// is used to specify the multiplier of multiple spacing values.
    /// 
    /// The function returns a vector of spacing values and can be empty, if no valid values are found.
pub fn parse_distribution_string(distribution: &str, equation_handler: &EquationHandler) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();
    let split = distribution.split(" ").collect::<Vec<&str>>();
    for s in split {
        // If the string contains '*' (e.g. 5*60) split that to multiplier and value and add them
        // to the result vector (60 60 60 60 60)
        if s.contains("*") {
            let split = s.split("*").collect::<Vec<&str>>();
            let multiplier = vputilslib::vputils::s_to_int(split[0]).unwrap_or(0);
            let value = equation_handler.calculate_formula(split[1]).unwrap_or(0.0);
            for _ in 0..multiplier {
                result.push(value);
            }
        } else {
            let value = equation_handler.calculate_formula(s).unwrap_or(0.0);
            if value.abs() > 0.0001 {
                result.push(value);
            }
        }
    }

    result
}


/// Calculates the weighted elastic centroid of a profile by the elastic modulus difference
/// of the rebar and concrete.
pub fn elastic_centroid(profile: &PolygonProfile, concrete: &Concrete, _calc_settings: &CalculationSettings) -> (f64, f64) {
    let ec = concrete.elastic_modulus;
    let mut total_area = 0.0;
    let mut result_x = 0.0;
    let mut result_y = 0.0;
    let centroid_concrete = vputilslib::geometry2d::centroid_from_polygon(&profile.polygon);
    let area_concrete = geometry2d::calculate_area(&profile.polygon);
    total_area += area_concrete;
    result_x += centroid_concrete.x * area_concrete;
    result_y += centroid_concrete.y * area_concrete;
    for r in &concrete.reinforcement.main_rebars {
        for s in r.get_calculation_rebars(profile, &EquationHandler::new()) {     
            let es = s.reinf_data.get_elastic_modulus();               
            // the -1 in '-1 + es / ec' is to take into account the area that the rebar 
            // 'takes' from concrete
            let s_area = s.area * (-1.0 + es / ec);
            result_x += s.x * s_area;
            result_y += s.y * s_area;
            total_area += s_area;
        }
    }    

    (result_x / total_area, result_y / total_area)
}

#[cfg(test)]
mod test {
    use crate::{material::StandardConcrete, profile::Profile, reinforcement::{ElementReinforcement, RebarCollection, RebarData, RebarDistribution, ReinforcementData}};

    use super::*;

    #[test]
    fn test_parse_distribution_string() {        
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("d", 0.0);
        assert_eq!(parse_distribution_string("5*60", &equation_handler), 
            vec![60.0, 60.0, 60.0, 60.0, 60.0]
        );
        equation_handler.set_variable("d", 0.0);
        assert!(parse_distribution_string("0 0 0 0", &equation_handler).is_empty());
        assert_eq!(parse_distribution_string("30 5*60 anc*123 30", &equation_handler), 
            vec![30.0, 60.0, 60.0, 60.0, 60.0, 60.0, 30.0]
        );
        equation_handler.set_variable("d", 25.0);
        assert_eq!(parse_distribution_string("30+d/2 5*60 anc*123 30", &equation_handler), 
            vec![42.5, 60.0, 60.0, 60.0, 60.0, 60.0, 30.0]
        );
    }

    #[test]
    fn test_elastic_centroid() {
        let profile = Profile::new_rectangle("name".to_string(), 580.0, 380.0);
        let mut concrete = Concrete::standard(StandardConcrete::C30_37);
        concrete.reinforcement.main_rebars.push(RebarCollection::new_bot_full(
            ReinforcementData::Rebar(RebarData::new_b500b()), 
            RebarDistribution::Even { diam: 25.0, count: 6, 
                cc_left: "30.0".to_string(), 
                cc_right: "30.0".to_string() }, 
            "30.0".to_string())
        );
        match profile {
            Profile::PolygonProfile(polygon_profile) =>  {
                let (x, y) = elastic_centroid(&polygon_profile, &concrete, &CalculationSettings::default());
                println!("X: {}, Y: {}", x, y);
                assert!((x-380.0/2.0).abs() < 0.01);
                assert!((y-274.235).abs() < 0.01);
            },
            Profile::StandardProfile(_) => panic!("Should be a polygon profile"),
            Profile::CustomProfile(_) => panic!("Should be a polygon profile"),
        }
    }

    #[test]
    fn test_elastic_centroid_rm() {
        // The example is from book Rakenteiden mekaniikka, Tapio Salmi, Kai Kuula, 2012
        let profile = Profile::new_rectangle("name".to_string(), 450.0, 300.0);
        let mut concrete = Concrete{
            elastic_modulus: 25e3,
            char_strength: 1.0,
            ..Default::default()
            };
        concrete.reinforcement.main_rebars.push(RebarCollection::new_bot_full(
            ReinforcementData::Rebar(RebarData::new(500.0, 210e3)), 
            RebarDistribution::Even { diam: 15.0, count: 1, 
                cc_left: "300/2-15/2".to_string(), 
                cc_right: "0".to_string() }, 
            "50-15/2".to_string())
        );
        let (x, y) = elastic_centroid(&profile.get_polygon_profile(), &concrete, &CalculationSettings::default());
        println!("X: {}, Y: {}", x, y);
        assert!((x-300.0/2.0).abs() < 0.01);
        assert!((y-223.0).abs() < 0.50);
    }
}