#![allow(dead_code)]

pub struct Steel {
    pub elastic_modulus: f64,
}

impl Steel {
    pub  fn new(elastic_modulus: f64) -> Self {
        Self { elastic_modulus }
    }
}