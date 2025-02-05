#[cfg(test)]
mod tests {
    use approx::relative_eq;
    use idfem::loads::Load;
    use idfem::material::Steel;
    use idfem::structure::element::MaterialType;
    use idfem::structure::Element;
    use idfem::structure::Node;
    use idfem::structure::Profile;
    use std::collections::HashMap;
    use std::mem;
    use std::ops::IndexMut;
    use vputilslib::equation_handler::EquationHandler;
    use vputilslib::geometry2d;
    use vputilslib::geometry2d::VpPoint;

    #[test]
    fn line_load() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let mut load = Load::new_line_load(
            "".to_string(),
            "1".to_string(),
            "0".to_string(),
            "4000".to_string(),
            "10".to_string(),
            -00.0,
        );
        let mut loads = vec![load];
        let mut equation_handler = EquationHandler::new();
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-2e4)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (13333333.0)).abs() < 1.0);
        assert!((result[3] - (-2e4)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (-13333333.0)).abs() < 1.0);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-20000.0)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (-20000.0)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-19.995e3)).abs() < 1e1);
        assert!((result[1] - (0.0)).abs() < 1e1);
        assert!((result[2] - (9.4281e6)).abs() < 1e3);
        assert!((result[3] - (-20.0e3)).abs() < 1e1);
        assert!((result[4] - (0.0e4)).abs() < 1e1);
        assert!((result[5] - (-9.4281e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-2.0e4)).abs() < 1e1);
        assert!((result[1] - (0.0)).abs() < 1e1);
        assert!((result[2] - (11.547e6)).abs() < 1e3);
        assert!((result[3] - (-2.0e4)).abs() < 1e1);
        assert!((result[4] - (0.0)).abs() < 1e1);
        assert!((result[5] - (-11.547e6)).abs() < 1e3);

        let load = Load::new_line_load(
            "".to_string(),
            "1".to_string(),
            "1000".to_string(),
            "3500".to_string(),
            "10".to_string(),
            0.0,
        );
        loads[0] = load;
        nodes.get_mut(&2).unwrap().point = VpPoint::new(0.0, 4000.0);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-10.4736e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (9.7493e6)).abs() < 1.0e2);
        assert!((result[3] - (-14.5264e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (-11.6048e6)).abs() < 1.0e2);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-10.9375e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (-14.0625e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-10.7024e3)).abs() < 1e1);
        assert!((result[1] - (-0.2288e3)).abs() < 1e1);
        assert!((result[2] - (6.8938e6)).abs() < 1e3);
        assert!((result[3] - (-14.2944e3)).abs() < 1e1);
        assert!((result[4] - (0.2319e3)).abs() < 1e1);
        assert!((result[5] - (-8.2058e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-10.5896e3)).abs() < 1e1);
        assert!((result[1] - (0.1982e3)).abs() < 1e1);
        assert!((result[2] - (8.4432e6)).abs() < 1e3);
        assert!((result[3] - (-14.41044e3)).abs() < 1e1);
        assert!((result[4] - (-0.2009e3)).abs() < 1e1);
        assert!((result[5] - (-10.0501e6)).abs() < 1e3);
    }

    #[test]
    fn triangular_load() {
        let el: Element = Element {
            number: 1,
            node_start: 1,
            node_end: 2,
            ..Element::default()
        };
        let mut nodes: HashMap<i32, Node> = HashMap::new();
        nodes.insert(1, Node::new_hinged(1, VpPoint::new(0.0, 0.0)));
        nodes.insert(2, Node::new_hinged(1, VpPoint::new(0.0, 4000.0)));
        let load = Load::new_triangular_load(
            "".to_string(),
            "1".to_string(),
            "0".to_string(),
            "4000".to_string(),
            "10".to_string(),
            -00.0,
        );
        let mut loads = vec![load];
        let mut equation_handler = EquationHandler::new();
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-14e3)).abs() < 0.1);
        assert!((result[1] - (0e3)).abs() < 0.1);
        assert!((result[2] - (8e6)).abs() < 1.0);
        assert!((result[3] - (-6e3)).abs() < 0.1);
        assert!((result[4] - (0e3)).abs() < 0.1);
        assert!((result[5] - (-5.333333e6)).abs() < 1.0);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-13.3333e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (-6.6667e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-13.6598e3)).abs() < 1e1);
        assert!((result[1] - (0.3298e3)).abs() < 1e1);
        assert!((result[2] - (5.6569e6)).abs() < 1e2);
        assert!((result[3] - (-6.3333e3)).abs() < 1e1);
        assert!((result[4] - (-0.3333e3)).abs() < 1e1);
        assert!((result[5] - (-3.7712e6)).abs() < 1e2);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-13.8246e3)).abs() < 1e1);
        assert!((result[1] - (-0.2856e3)).abs() < 1e1);
        assert!((result[2] - (6.9282e6)).abs() < 1e3);
        assert!((result[3] - (-6.1629e3)).abs() < 1e1);
        assert!((result[4] - (0.2899e3)).abs() < 1e1);
        assert!((result[5] - (-4.6188e6)).abs() < 1e3);

        let load = Load::new_triangular_load(
            "".to_string(),
            "1".to_string(),
            "1000".to_string(),
            "3500".to_string(),
            "10".to_string(),
            0.0,
        );
        loads[0] = load;
        nodes.get_mut(&2).unwrap().point = VpPoint::new(0.0, 4000.0);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-7.0068e3)).abs() < 0.1);
        assert!((result[1] - (0.0e3)).abs() < 0.1);
        assert!((result[2] - (6.1361e6)).abs() < 1.0e2);
        assert!((result[3] - (-5.4931e3)).abs() < 0.1);
        assert!((result[4] - (0.0e3)).abs() < 0.1);
        assert!((result[5] - (-5.1921e6)).abs() < 1.0e2);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(4000.0, 0.0);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-6.7708e3)).abs() < 0.1);
        assert!((result[1] - (0.0)).abs() < 0.1);
        assert!((result[2] - (0.0)).abs() < 0.1);
        assert!((result[3] - (-5.7292e3)).abs() < 0.1);
        assert!((result[4] - (0.0)).abs() < 0.1);
        assert!((result[5] - (0.0)).abs() < 0.1);

        nodes.get_mut(&2).unwrap().point = VpPoint::new(2828.5714, 2828.5714);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-6.8839e3)).abs() < 1e1);
        assert!((result[1] - (0.1162e3)).abs() < 1e1);
        assert!((result[2] - (4.3389e6)).abs() < 1e3);
        assert!((result[3] - (-5.6099e3)).abs() < 1e1);
        assert!((result[4] - (-0.1193e3)).abs() < 1e1);
        assert!((result[5] - (-3.6714e6)).abs() < 1e3);

        // 120°
        nodes.get_mut(&2).unwrap().point = VpPoint::new(-2000.0, 3464.10161513775458);
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-6.9392e3)).abs() < 1e1);
        assert!((result[1] - (-0.099e3)).abs() < 1e1);
        assert!((result[2] - (5.3139e6)).abs() < 1e3);
        assert!((result[3] - (-5.5476e3)).abs() < 1e1);
        assert!((result[4] - (0.1036e3)).abs() < 1e1);
        assert!((result[5] - (-4.4964e6)).abs() < 1e3);

        // 120°
        loads[0].offset_start = "3500".to_string();
        loads[0].offset_end = "1000".to_string();
        let result = idfem::fem::equivalent_loads::get_element_global_equivalent_loads(
            &el,
            &loads,
            &nodes,
            &mut equation_handler,
        );
        println!("{:?}", result);
        assert!((result[0] - (-3.64176e3)).abs() < 1e1);
        assert!((result[1] - (0.3042e3)).abs() < 1e1);
        assert!((result[2] - (3.1292e6)).abs() < 1e3);
        assert!((result[3] - (-8.8582e3)).abs() < 1e1);
        assert!((result[4] - (-0.3007e3)).abs() < 1e1);
        assert!((result[5] - (-5.5536e6)).abs() < 1e3);
    }
}
