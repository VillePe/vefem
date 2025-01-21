pub struct Concrete {
    pub elastic_modulus: f64,
}

impl Concrete {
    pub  fn new(elastic_modulus: f64) -> Concrete {
        Concrete { elastic_modulus }
    }
}