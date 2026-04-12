pub mod element;
mod node;
mod release;
mod support;
mod structure_model;
pub mod utils;

pub type NodeCollection = BTreeMap<i32, Node>;

use std::collections::BTreeMap;
pub use element::Element;
pub use node::Node;
pub use release::Release;
pub use support::Support;
pub use structure_model::StructureModel;
pub use element::CalculationElement;