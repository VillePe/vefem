# Vefem

Vefem is a simple FEA library to calculate the displacements, support reactions and internal forces
for simple beam elements in 2D space.

The library is still in a really early stage and should not be used anywhere else than testing environments.
It would be preferred to run additional tests with robust FEA analysis programs. Do not use this library
to analyse real world problems!

## Table of Contents

- [Vefem](#vefem)
  - [Getting started](#getting-started)
  - [License](#license)

## Getting started

Add the dependency to your Cargo.toml
```rust
[dependencies]
vefem = { git = "https://github.com/VillePe/vefem" }
```

An example how calculate a single span beam with line load on the beam.
```rust
use std::collections::BTreeMap;

use vefem::vputilslib::{equation_handler::EquationHandler, geometry2d::VpPoint};
use vefem::{
    loads,
    material::{MaterialData, Steel},
    profile::Profile,
    settings::CalculationSettings,
    structure::{CalculationModel, Node},
};

fn test_vefem() {
    let n1 = Node::new_hinged(1, VpPoint::new(0.0, 0.0));
    let n2 = Node::new_hinged(2, VpPoint::new(4000.0, 0.0));
    let nodes = BTreeMap::from([(n1.number, n1), (n2.number, n2)]);
    let el = vefem::structure::Element::new(
        1, // Element number
        1, // The node number at the start of the element
        2, // The node number at the end of the element
        Profile::new_rectangle("R100x100".to_string(), 100.0, 100.0),
        MaterialData::Steel(Steel::new(210e3)),
    );
    let elements = vec![el];
    let line_load = loads::Load::new_line_load(
        "LineLoad".to_string(),
        "1".to_string(),  // Element number(s)
        "0".to_string(),  // The offset of the loads start from the start of the element
        "L".to_string(),  // The offset of the loads end from the start of the element
        "10".to_string(), // in N/mm
        -90.0,
    ); // 0.0 points towards positive X-axis and goes counter clockwise
    let loads = vec![line_load];
    let mut eq_handler = EquationHandler::new();
    let calc_settings = CalculationSettings::default();
    let calc_model = CalculationModel {
        nodes,
        elements,
        loads,
        calc_settings,
        load_combinations: vec![],
    };
    let results = vefem::fem::calculate(&calc_model, &mut eq_handler);
    // The default settings divide the internal force calculation points into 100 intervals.
    // Assert that the value at the middle of the element is ql^2/8
    assert_eq!(
        results.internal_force_results[&1].moment_forces[50].value_y,
        10.0 * 4000f64.powi(2) / 8.0
    );
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

