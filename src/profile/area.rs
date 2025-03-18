pub fn area_from_radius(radius: f64) -> f64 {
    std::f64::consts::PI * radius.powi(2)
}

pub fn area_from_diameter(diameter: f64) -> f64 {
    std::f64::consts::PI * diameter.powi(2) / 4.0
}