pub struct CalculationSettings {
    pub calc_split_interval: CalcSplitInterval,
}

pub enum CalcSplitInterval {
    /// Splits the calculation into intervals of a fixed length.
    Absolute(f64),
    /// Splits the calculation into intervals relative to the length of the element.
    Relative(f64),
}

impl Default for CalculationSettings {
    fn default() -> Self {
        Self {
            calc_split_interval: CalcSplitInterval::Relative(0.01),
        }
    }
}