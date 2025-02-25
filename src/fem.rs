#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod fem_handler;
pub mod matrices;
pub mod stiffness;
pub mod equivalent_loads;
pub mod utils;
pub mod internal_forces;
pub mod deflection;
pub mod node_results;

pub use node_results::NodeResults;