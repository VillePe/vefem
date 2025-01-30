#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::HashMap;
use crate::structure::element::Element;
use nalgebra::{DMatrix};
use crate::structure::node::Node;

mod fem_handler;
mod matrices;

fn create_joined_stiffness_matrix(elements: Vec<Element>, nodes: &HashMap<i32, Node>) -> DMatrix<f64> {
    let supp_count = nodes.len();
    let row_width = supp_count * 3;
    // println!("Row width: {}", row_width);
    let mut matrix_vector = DMatrix::from_row_slice(row_width, row_width, &vec![0.0; supp_count*supp_count*3*3]);
    for elem in elements {
        let e_glob_stiff_matrix = matrices::get_element_global_stiffness_matrix(&elem, nodes);
        // The index of the start node
        let s = (elem.node_start-1) as usize;
        // The index of the end node
        let e = (elem.node_end-1) as usize;
        for i in 0..6 {
            for j in 0..6 {
                // println!("s={}, e={}", s, e);
                if i < 3 {
                    if j < 3 {
                        matrix_vector[(s*3)*row_width+s*3+j+i*row_width] += e_glob_stiff_matrix[(i,j)];
                        // println!("#1: i={i},j={j}: {}, {:.02e}", (s*3)*row_width+s*3+j+i*row_width, (e_glob_stiff_matrix[(i,j)]*1000.0).round()/1000.0);
                    } else {
                        matrix_vector[(s*3)*row_width+e*3+(j-3)+i*row_width] += e_glob_stiff_matrix[(i,j)];
                        // println!("#2: i={i},j={j}: {}, {:9.02e}", (s*3)*row_width+e*3+(j-3)+i*row_width, (e_glob_stiff_matrix[(i,j)]*1000.0).round()/1000.0);
                    }
                } else {
                    if j < 3 {
                        matrix_vector[(e*3)*row_width+s*3+j+(i-3)*row_width] += e_glob_stiff_matrix[(i,j)];
                        // println!("#3: i={i},j={j}: {}, {:9.02e}", (e*3)*row_width+s*3+j+(i-3)*row_width, (e_glob_stiff_matrix[(i,j)]*1000.0).round()/1000.0);
                    } else {
                        matrix_vector[(e*3)*row_width+e*3+(j-3)+(i-3)*row_width] += e_glob_stiff_matrix[(i,j)];
                        // println!("#4: i={i},j={j}: {}, {:9.02e}", (e*3)*row_width+e*3+(j-3)+(i-3)*row_width, (e_glob_stiff_matrix[(i,j)]*1000.0).round()/1000.0);
                    }
                }
            }
        }
    }

    matrix_vector
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
    use crate::structure::element::Material;

    #[test]
    fn rotated_stiffness_matrix() {
        let end_point: VpPoint = geometry2d::rotate_point(
            VpPoint::new(0.0, 0.0),
            VpPoint::new(8000.0, 0.0),
            22.0243128370,
        );

        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, end_point));

        let e: Element = Element::new(
            1,
            2,
            Profile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 200_000_000.0,
                custom_area: 6000.0,
                ..Profile::default()
            },
            Material::Steel(Steel::new(200.0)),
        );
        let e_glob_stiff_matrix = matrices::get_element_global_stiffness_matrix(&e, &nodes) / 200.0;
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
    fn joined_stiffness_matrix_FES() {
        let end_point: VpPoint = geometry2d::rotate_point(
            VpPoint::new(0.0, 0.0),
            VpPoint::new(8000.0, 0.0),
            22.0243128370,
        );

        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(2, end_point));
        nodes.insert(3, Node::new_hinged(3, VpPoint::new(nodes[&2].point.x+8000.0, 3000.0)));

        let e1: Element = Element::new(
            1,
            2,
            Profile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 200_000_000.0,
                custom_area: 6000.0,
                ..Profile::default()
            },
            Material::Steel(Steel::new(200.0)),
        );
        let e2: Element = Element::new(
            2,
            3,
            Profile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 200_000_000.0,
                custom_area: 6000.0,
                ..Profile::default()
            },
            // Note the elastic modulus of 200. Comes from the source material
            Material::Steel(Steel::new(200.0)),
        );
        let joined = create_joined_stiffness_matrix(vec![e1, e2], &nodes);
        for i in 0..9 {
            for j in 0..9 {
                // Note the multiplication of 200. Comes from the source material
                print!("{:>12.04}, ", joined[(i,j)]/200.0);
            }
            println!();
        }

        // Test the matrix cells that overlap (the same node for both elements)
        // Note the multiplication of 200. Comes from the source material
        assert!(relative_eq!(joined[(3,3)], 1.3952*200.0, max_relative=0.001));
        assert!(relative_eq!(joined[(4,3)], 0.2591*200.0, max_relative=0.001));
        assert!(relative_eq!(joined[(5,3)], 7.0312*200.0, max_relative=0.001));
        assert!(relative_eq!(joined[(3,4)], 0.2591*200.0, max_relative=0.001));
        assert!(relative_eq!(joined[(4,4)], 0.1142*200.0, max_relative=0.001));
        assert!(relative_eq!(joined[(5,4)], 1.3683*200.0, max_relative=0.001));
        assert!(relative_eq!(joined[(3,5)], 7.0312*200.0, max_relative=0.001));
        assert!(relative_eq!(joined[(4,5)], 1.3683*200.0, max_relative=0.001));
        assert!(relative_eq!(joined[(5,5)], 2.0e5*200.0, max_relative=0.001));
    }

    #[test]
    fn joined_stiffness_matrix_Harj() {
        // Based on Harjoitustyö by Savonia AMK

        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(2, VpPoint::new(0.0, 4000.0)));
        nodes.insert(3, Node::new_hinged(3, VpPoint::new(nodes[&2].point.x+6000.0, 0.0)));
        nodes.insert(4, Node::new_hinged(4, VpPoint::new(nodes[&3].point.x, nodes[&2].point.y)));
        

        let e1: Element = Element::new(
            1,
            2,
            Profile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 7.763e07,
                custom_area: 7.680e03,
                ..Profile::default()
            },
            Material::Steel(Steel::new(210e3)),
        );
        let e2: Element = Element::new(
            2,
            4,
            Profile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 1.412e09,
                custom_area: 2.265e04,
                ..Profile::default()
            },
            Material::Steel(Steel::new(210e3)),
        );
        let e3: Element = Element::new(
            4,
            3,
            Profile {
                name: "TEST".to_string(),
                custom_major_sec_mom_of_area: 7.763e07,
                custom_area: 7.680e03,
                ..Profile::default()
            },
            Material::Steel(Steel::new(210e3)),
        );
        
        let joined = create_joined_stiffness_matrix(vec![e1, e2, e3], &nodes);
        // for i in 0..12 {
        //     for j in 0..12 {
        //         let val = joined[(i,j)];
        //         if val.abs() < 0.001 {
        //             print!("{:>9.0}, ",  joined[(i,j)]);
        //         } else {
        //             print!("{:>9.02e}, ", joined[(i,j)]);
        //         }
        //     }
        //     println!();
        // }

        // Test the matrix cells that overlap (the same node for both elements)
        assert!(relative_eq!(joined[(3,3)], 7.96e05, max_relative=0.01));
        assert!(            (joined[(4,3)].round() == 0.0));
        assert!(relative_eq!(joined[(5,3)], 6.11e06, max_relative=0.01));
        assert!(            (joined[(3,4)].round() == 0.0));
        assert!(relative_eq!(joined[(4,4)], 4.20e05, max_relative=0.01));
        assert!(relative_eq!(joined[(5,4)], 4.94e07, max_relative=0.01));
        assert!(relative_eq!(joined[(3,5)], 6.11e06, max_relative=0.01));
        assert!(relative_eq!(joined[(4,5)], 4.94e07, max_relative=0.01));
        assert!(relative_eq!(joined[(5,5)], 2.14e11, max_relative=0.01));

        assert!(relative_eq!(joined[(9,9)], 7.96e05, max_relative=0.01));
        assert!(            (joined[(10,9)].round() == 0.0));
        assert!(relative_eq!(joined[(11,9)], 6.11e06, max_relative=0.01));
        assert!(            (joined[(9,10)].round() == 0.0));
        assert!(relative_eq!(joined[(10,10)], 4.20e05, max_relative=0.01));
        assert!(relative_eq!(joined[(11,10)], -4.94e07, max_relative=0.01));
        assert!(relative_eq!(joined[(9,11)], 6.11e06, max_relative=0.01));
        assert!(relative_eq!(joined[(10,11)], -4.94e07, max_relative=0.01));
        assert!(relative_eq!(joined[(11,11)], 2.14e11, max_relative=0.01));
    }
}
