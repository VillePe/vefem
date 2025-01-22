#![allow(non_snake_case)]

use nalgebra::{DMatrix, Matrix};
use crate::structure::{element::Element};
use crate::structure::element::Material;

mod fem_handler;

fn create_joined_stiffness_matrix(elements: Vec<Element>, supp_count: usize) -> DMatrix<f64> {
    
    let matrix_vector : Vec<f64> = Vec::with_capacity(supp_count);
    for e in elements {
        
    }
    
    DMatrix::from_row_slice(6,6, &[
        0.0, 0.1, 0.2, 0.3, 0.4, 0.5,
        1.0, 1.1, 1.0, 1.0, 1.0, 1.0,
        2.0, 0.0, 2.2, 0.0, 0.0, 0.0,
        3.0, 0.0, 0.0, 3.3, 0.0, 0.0,
        4.0, 0.0, 0.0, 0.0, 4.4, 0.0,
        5.0, 0.0, 0.0, 0.0, 0.0, 5.5,
    ])
}

fn get_element_global_stiffness_matrix(element: Element) -> DMatrix<f64> {
    let E = match &element.material {
        Material::Concrete(c) => {c.elastic_modulus}
        Material::Steel(_) => {0.0}
        Material::Timber(_) => {0.0}
    };
    let L = element.get_length();
    let A = element.profile.get_area();
    let I = element.profile.get_major_mom_of_inertia();
    let EA = E*A;
    let EI = E*I;
    DMatrix::from_row_slice(6,6, &[
        EA/L,  0.,                 0.,                -EA/L, 0.,                 0.,
        0.0,   12.0*EI/L.powi(3),  6.0*EI/L.powi(2),  0.0,   -12.0*EI/L.powi(3), 6.0*EI/L.powi(2),
        0.0,   6.0*EI/L.powi(2),   4.0*EI/L,          0.0,   -6.0*EI/L.powi(2),  2.0*EI/L,
        -EA/L, 0.0,                0.0,               EA/L,  0.0,                0.0,
        0.0,   -12.0*EI/L.powi(3), -6.0*EI/L.powi(2), 0.0,   12.0*EI/L.powi(3),  -6.0*EI/L.powi(2),
        0.0,   6.0*EI/L.powi(2),   2.0*EI/L,          0.0,   -6.0*EI/L.powi(2),  4.0*EI/L,
    ])
}

#[cfg(test)]
mod tests {
    use vputilslib::geometry2d::VpPoint;
    use crate::material::concrete;
    use crate::structure::element::Material::Concrete;
    use crate::structure::node::Support;
    use crate::structure::node::Node;
    use crate::structure::profile::Profile;
    use super::*;

    #[test]
    fn joined_stiffness_matrix() {
        let ss = Node::new(VpPoint::new(0.0,0.0), Support::new_hinged());
        let es = Node::new(VpPoint::new(4000.0,0.0), Support::new_hinged());
        let p = Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0);
        let v = vec![Element::new(ss, es, p, Concrete(concrete::Concrete::new(37000.0)))];
        let matrix = create_joined_stiffness_matrix(v, 2);
        println!("{:?}", matrix);
        let m11 = matrix[(0,0)];
        let m66 = matrix[(5,0)];
        println!("{:?}", m11);
        println!("{:?}", m66);
    }
}