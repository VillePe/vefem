#![allow(non_snake_case)]

use crate::structure::element::Element;
use crate::structure::element::Material;
use nalgebra::{DMatrix, Matrix};

mod fem_handler;
mod matrices;

fn create_joined_stiffness_matrix(elements: Vec<Element>, supp_count: usize) -> DMatrix<f64> {
    let row_width = supp_count * 3;
    let mut matrix_vector = DMatrix::from_row_slice(row_width, row_width, &vec![0.0; supp_count*supp_count*3*3]);
    for e in elements {
        let e_glob_stiff_matrix = get_global_stiffness_matrix(&e);
        // The index of the start node
        let s = (e.node_start.number-1) as usize;
        // The index of the end node
        let e = (e.node_end.number-1) as usize;
        for i in 0..6 {
            for j in 0..6 {
                if i < 3 {
                    if j < 3 {
                        matrix_vector[(s*3)*row_width+s*3+j+i*row_width] += e_glob_stiff_matrix[(i,j)];
                        println!("#1: i={i},j={j}: {}, {}", (s*3)*row_width+s*3+j+i*row_width, e_glob_stiff_matrix[(i,j)]/200.0);
                    } else {
                        matrix_vector[(s*3)*row_width+e*3+(j-3)+i*row_width] += e_glob_stiff_matrix[(i,j)];
                        println!("#2: i={i},j={j}: {}, {}", (s*3)*row_width+e*3+(j-3)+i*row_width, e_glob_stiff_matrix[(i,j)]/200.0);
                    }
                } else {
                    if j < 3 {
                        matrix_vector[(e*3)*row_width+s*3+j+(i-3)*row_width] += e_glob_stiff_matrix[(i,j)];
                        println!("#3: i={i},j={j}: {}, {}", (e*3)*row_width+s*3+j+(i-3)*row_width, e_glob_stiff_matrix[(i,j)]/200.0);
                    } else {
                        matrix_vector[(e*3)*row_width+e*3+(j-3)+(i-3)*row_width] += e_glob_stiff_matrix[(i,j)];
                        println!("#4: i={i},j={j}: {}, {}", (e*3)*row_width+e*3+(j-3)+(i-3)*row_width, e_glob_stiff_matrix[(i,j)]/200.0);
                    }
                }
            }
        }
    }

    matrix_vector
}

fn get_global_stiffness_matrix(e: &Element) -> DMatrix<f64> {
    let e_stiff_matrix = matrices::get_element_global_stiffness_matrix(&e);
    let e_rotation_matrix = matrices::get_element_rotation_matrix(&e);
    let e_rot_matrix_T = e_rotation_matrix.transpose();
    let e_glob_stiff_matrix = e_rot_matrix_T * e_stiff_matrix * e_rotation_matrix;
    e_glob_stiff_matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::steel::Steel;
    use crate::structure::node::Node;
    use crate::structure::profile::Profile;
    use approx::relative_eq;
    use vputilslib::geometry2d;
    use vputilslib::geometry2d::VpPoint;

    #[test]
    fn rotated_stiffness_matrix() {
        let end_point: VpPoint = geometry2d::rotate_point(
            VpPoint::new(0.0, 0.0),
            VpPoint::new(8000.0, 0.0),
            22.0243128370,
        );

        let e: Element = Element::new(
            Node::new_hinged(1, VpPoint::new(0.0, 0.0)),
            Node::new_hinged(2, end_point),
            Profile {
                name: "TEST".to_string(),
                custom_major_mom_of_inertia: 200_000_000.0,
                custom_area: 6000.0,
                ..Profile::default()
            },
            Material::Steel(Steel::new(200.0)),
        );
        let e_glob_stiff_matrix = get_global_stiffness_matrix(&e) / 200.0;
        println!("{}", e_glob_stiff_matrix);
        assert!(relative_eq!(e_glob_stiff_matrix[(0,0)], 0.6451, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(0,1)], 0.2590, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(0,2)], -7.0312, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(0,3)], -0.6451, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(0,4)], -0.2590, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(0,5)], -7.0312, max_relative=0.001));

        assert!(relative_eq!(e_glob_stiff_matrix[(1,0)], 0.2590, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(1,1)], 0.1094, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(1,2)], 17.3817, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(1,3)], -0.2590, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(1,4)], -0.1094, max_relative=0.001));

        assert!(relative_eq!(e_glob_stiff_matrix[(2,0)], -7.0312, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(2,2)], 1e5, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(2,3)], 7.0312, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(2,5)], 0.5e5, max_relative=0.001));

        assert!(relative_eq!(e_glob_stiff_matrix[(5,0)], -7.0312, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(5,1)], 17.3817, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(5,2)], 0.5e5, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(5,4)], -17.3817, max_relative=0.001));
        assert!(relative_eq!(e_glob_stiff_matrix[(5,5)], 1e5, max_relative=0.001));
    }

    #[test]
    fn joined_stiffness_matrix() {
        let end_point: VpPoint = geometry2d::rotate_point(
            VpPoint::new(0.0, 0.0),
            VpPoint::new(8000.0, 0.0),
            22.0243128370,
        );

        let e1: Element = Element::new(
            Node::new_hinged(1, VpPoint::new(0.0, 0.0)),
            Node::new_hinged(2, end_point),
            Profile {
                name: "TEST".to_string(),
                custom_major_mom_of_inertia: 200_000_000.0,
                custom_area: 6000.0,
                ..Profile::default()
            },
            Material::Steel(Steel::new(200.0)),
        );
        let e2: Element = Element::new(
            Node::new_hinged(2, VpPoint::new(e1.node_end.point.x, e1.node_end.point.y)),
            Node::new_hinged(3, VpPoint::new(e1.node_end.point.x+8000.0, 3000.0)),
            Profile {
                name: "TEST".to_string(),
                custom_major_mom_of_inertia: 200_000_000.0,
                custom_area: 6000.0,
                ..Profile::default()
            },
            Material::Steel(Steel::new(200.0)),
        );
        let joined = create_joined_stiffness_matrix(vec![e1, e2], 3);
        for i in 0..9 {
            for j in 0..9 {
                print!("{:>12.04}, ", joined[(i,j)]/200.0);
            }
            println!();
        }
    }
}
