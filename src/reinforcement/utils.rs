use vputilslib::equation_handler::EquationHandler;

    /// Parses a distribution string into a vector of spacing values. The string is formatted with
    /// space separated values and can contain values with '*' character (e.g. 5*60). The '*' character
    /// is used to specify the multiplier of multiple spacing values.
    /// 
    /// The parser uses an EquationHandler so the string can contain the 'd' or 'Ø' (alt + 0216 or U+00D8)
    /// characters to refer to the diameter value.
    /// 
    /// The function returns a vector of spacing values and can be empty, if no valid values are found.
pub fn parse_distribution_string(diam: f64, distribution: &str) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    let equation_handler = EquationHandler::from([("d", diam), ("Ø", diam)]);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_distribution_string() {        
        assert_eq!(parse_distribution_string(0.0, "5*60"), 
            vec![60.0, 60.0, 60.0, 60.0, 60.0]
        );
        assert!(parse_distribution_string(0.0, "0 0 0 0").is_empty());
        assert_eq!(parse_distribution_string(0.0, "30 5*60 anc*123 30"), 
            vec![30.0, 60.0, 60.0, 60.0, 60.0, 60.0, 30.0]
        );
        assert_eq!(parse_distribution_string(25.0, "30+d/2 5*60 anc*123 30"), 
            vec![42.5, 60.0, 60.0, 60.0, 60.0, 60.0, 30.0]
        );
    }
}