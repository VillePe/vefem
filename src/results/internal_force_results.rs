use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InternalForceResults {
    /// The element number to which the internal forces are linked
    pub element_number: i32,
    pub axial_forces: Vec<InternalForcePoint>,
    pub shear_forces: Vec<InternalForcePoint>,
    pub moment_forces: Vec<InternalForcePoint>,
    pub deflections: Vec<InternalForcePoint>
}

#[derive(Serialize, Deserialize)]
pub struct InternalForcePoint {
    /// The force type of the internal force
    pub force_type: ForceType,
    /// The value of the internal force in direction X in elements local coordinates (usually only used in deflection)
    pub value_x: f64,
    /// The value of the internal force in direction Y in elements local coordinates (the more commonly used than x-dir)
    pub value_y: f64,
    /// The position on the element measured from the start of the element
    pub pos_on_element: f64,
    /// The element number to which the internal force is linked
    pub element_number: i32,
    /// The load combination number to which the internal force is linked
    pub load_comb_number: i32,
}

#[derive(Serialize, Deserialize)]
pub enum ForceType {
    Axial,
    Shear,
    Moment,
    Deflection
}