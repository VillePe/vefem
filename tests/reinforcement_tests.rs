mod common;

#[cfg(test)]
mod reinf_tests {
    use crate::common::get_inversed_t_profile;
    use vefem::{profile::Profile, reinforcement::{reinforcement::{RebarDistribution, ReinforcementData}, 
    RebarCollection, RebarData, Side}};
    use vputilslib::{equation_handler::EquationHandler, geometry2d::{Polygon, VpPoint}};

    #[test]
    fn test_get_single_rebars_bbox() {
        let side = Side::BoundingBox { index: 0 };
        let profile = Profile::new_rectangle("name".to_string(), 480.0, 280.0);
        let reinf_data = ReinforcementData::Rebar(RebarData {
            yield_strength: 500.0,
            elastic_modulus: 200e3,
        });
        let offset_start = "0".to_string();
        let offset_end = "L".to_string();
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("L", 4000.0);

        let rebar_coll1: RebarCollection = RebarCollection {
            reinf_data,
            offset_start,
            offset_end,
            concrete_cover: 30.0,
            side,
            distribution: RebarDistribution::Even {
                diam: 16.0,
                count: 4,
                cc_left: "30".to_string(),
                cc_right: "30".to_string(),
            },
        };
        let result1 = rebar_coll1.get_single_rebars(&profile, &equation_handler);
        for rebar in &result1 {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!(result1[0].x == 38.0);
        assert!(result1[1].x == 106.0);
        assert!(result1[2].x == 174.0);
        assert!(result1[3].x == 242.0);
    }

    #[test]
    fn test_get_single_rebars_polygon() {
        let side = Side::Polygon { index: 0 };
        let profile = Profile::new_rectangle("name".to_string(), 480.0, 280.0);
        let reinf_data = ReinforcementData::Rebar(RebarData {
            yield_strength: 500.0,
            elastic_modulus: 200e3,
        });
        let offset_start = "0".to_string();
        let offset_end = "L".to_string();
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("L", 4000.0);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side,
            distribution: RebarDistribution::Even {
                diam: 16.0,
                count: 4,
                cc_left: "30".to_string(),
                cc_right: "30".to_string(),
            },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!(result[0].x == 38.0);
        assert!(result[1].x == 106.0);
        assert!(result[2].x == 174.0);
        assert!(result[3].x == 242.0);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side: Side::Polygon { index: 1 },
            distribution: RebarDistribution::Even {
                diam: 16.0,
                count: 4,
                cc_left: "30".to_string(),
                cc_right: "30".to_string(),
            },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {:.2}, Y: {:.2}", rebar.x, rebar.y);
        }
        assert!((result[0].y - 38.00).abs() < 0.01);
        assert!((result[1].y - 172.67).abs() < 0.01);
        assert!((result[2].y - 307.33).abs() < 0.01);
        assert!((result[3].y - 442.00).abs() < 0.01);
    }

    #[test]
    fn test_get_single_rebars_polygon_clockwise() {
        let profile = Profile::new("name".to_string(), Polygon::new(vec![
            VpPoint::new(280.0, 0.0),
            VpPoint::new(0.0, 0.0),
            VpPoint::new(0.0, 480.0),
            VpPoint::new(280.0, 480.0),
            VpPoint::new(280.0, 0.0),
        ]));
        let reinf_data = ReinforcementData::Rebar(RebarData {
            yield_strength: 500.0,
            elastic_modulus: 200e3,
        });
        let offset_start = "0".to_string();
        let offset_end = "L".to_string();
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("L", 4000.0);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side: Side::Polygon { index: 1 },
            distribution: RebarDistribution::Even {
                diam: 16.0,
                count: 4,
                cc_left: "30".to_string(),
                cc_right: "30".to_string(),
            },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!((result[0].y - 38.00).abs() < 0.01);
        assert!((result[1].y - 172.67).abs() < 0.01);
        assert!((result[2].y - 307.33).abs() < 0.01);
        assert!((result[3].y - 442.00).abs() < 0.01);
    }

    #[test]
    fn test_get_single_rebars_polygon_inverted_t() {
        let profile = get_inversed_t_profile();
        let reinf_data = ReinforcementData::Rebar(RebarData {
            yield_strength: 500.0,
            elastic_modulus: 200e3,
        });
        let offset_start = "0".to_string();
        let offset_end = "L".to_string();
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("L", 4000.0);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side: Side::Polygon { index: 0 },
            distribution: RebarDistribution::Even {
                diam: 25.0,
                count: 6,
                cc_left: "30".to_string(),
                cc_right: "30".to_string(),
            },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!((result[0].x - 42.50).abs() < 0.01);
        assert!((result[1].x - 201.50).abs() < 0.01);
        assert!((result[2].x - 360.50).abs() < 0.01);
        assert!((result[3].x - 519.50).abs() < 0.01);
        assert!((result[4].x - 678.50).abs() < 0.01);
        assert!((result[5].x - 837.50).abs() < 0.01);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side: Side::Polygon { index: 4 }, // On the top
            distribution: RebarDistribution::Even {
                diam: 25.0,
                count: 6,
                cc_left: "30".to_string(),
                cc_right: "30".to_string(),
            },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!((result[0].y - (580.-30.-25./2.)).abs() < 0.01);

        // Notice the reversed order of indices
        assert!((result[5].x - (42.50+200.)).abs() < 0.01);
        assert!((result[4].x - (121.50+200.)).abs() < 0.01);
        assert!((result[3].x - (200.50+200.)).abs() < 0.01);
        assert!((result[2].x - (279.50+200.)).abs() < 0.01);
        assert!((result[1].x - (358.50+200.)).abs() < 0.01);
        assert!((result[0].x - (437.50+200.)).abs() < 0.01);
    }

    #[test]
    fn test_get_single_rebars_polygon_inverted_t_distr() {
        let profile = get_inversed_t_profile();
        let reinf_data = ReinforcementData::Rebar(RebarData {
            yield_strength: 500.0,
            elastic_modulus: 200e3,
        });
        let offset_start = "0".to_string();
        let offset_end = "L".to_string();
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("L", 4000.0);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side: Side::Polygon { index: 0 },
            distribution: RebarDistribution::Distributed { diam: 20.0, distr: "30+d/2 60 60 60 60 60".to_string() },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!((result[0].x - 40.00).abs() < 0.01);
        assert!((result[1].x - 100.00).abs() < 0.01);
        assert!((result[2].x - 160.00).abs() < 0.01);
        assert!((result[3].x - 220.00).abs() < 0.01);
        assert!((result[4].x - 280.00).abs() < 0.01);
        assert!((result[5].x - 340.00).abs() < 0.01);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side: Side::Polygon { index: 4 }, // On the top
            distribution: RebarDistribution::Distributed { diam: 20.0, distr: "30+d/2 60 60 60 60 60".to_string() },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!((result[0].y - (580.-30.-20./2.)).abs() < 0.01);

        // Notice the reversed order of indices
        assert!((result[0].x - (680.0-40.00)).abs() < 0.01);
        assert!((result[1].x - (680.0-100.00)).abs() < 0.01);
        assert!((result[2].x - (680.0-160.00)).abs() < 0.01);
        assert!((result[3].x - (680.0-220.00)).abs() < 0.01);
        assert!((result[4].x - (680.0-280.00)).abs() < 0.01);
        assert!((result[5].x - (680.0-340.00)).abs() < 0.01);
    }

    #[test]
    fn test_get_single_rebars_polygon_inverted_t_single() {
        let profile = get_inversed_t_profile();
        let reinf_data = ReinforcementData::Rebar(RebarData {
            yield_strength: 500.0,
            elastic_modulus: 200e3,
        });
        let offset_start = "0".to_string();
        let offset_end = "L".to_string();
        let mut equation_handler = EquationHandler::new();
        equation_handler.add_variable("L", 4000.0);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side: Side::Polygon { index: 0 },
            distribution: RebarDistribution::Single { diam: 20.0, off_left: "30+d/2".to_string(), off_bot: "30+d/2".to_string() },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!((result[0].x - 40.00).abs() < 0.01);
        assert!((result[0].y - 40.00).abs() < 0.01);

        let rebar_coll: RebarCollection = RebarCollection {
            reinf_data,
            offset_start: offset_start.clone(),
            offset_end: offset_end.clone(),
            concrete_cover: 30.0,
            side: Side::Polygon { index: 4 }, // On the top
            distribution: RebarDistribution::Single { diam: 20.0, off_left: "30+d/2".to_string(), off_bot: "30+d/2".to_string() },
        };
        let result = rebar_coll.get_single_rebars(&profile, &equation_handler);
        for rebar in &result {
            println!("X: {}, Y: {}", rebar.x, rebar.y);
        }
        assert!((result[0].x - 40.00).abs() < 0.01);
        assert!((result[0].y - 40.00).abs() < 0.01);
    }
}