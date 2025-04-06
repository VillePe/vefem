use crate::loads::load::LoadType::{
    Line, Point, Rotational, Strain, Thermal, Trapezoid, Triangular,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use vputilslib::equation_handler::EquationHandler;

use super::load_group::LoadGroup;

#[derive(Debug, Serialize, Deserialize)]
pub struct Load {
    /// Name for the load. Does not need to be unique. Load combinations are created by using the load names.
    pub name: String,
    /// The numbers for which this load is applied to. Different elements are separated by , (comma)
    /// can be used with <S..E> (double dots with numbers before and after it) to imply that elements
    /// from 'S' to 'E' are included (the element with number 'E' is also included)
    ///
    /// If all load should be linked to all elements, -1 can be used (e.g. for self weight load)
    pub element_numbers: String,
    /// The load type for the load. Controls how the load needs to be handled.
    pub load_type: LoadType,
    /// The offset for the load measured from the start of the element to the start of the load.
    /// Always in the X-axis of the elements local coordinates.
    pub offset_start: String,
    /// The offset for the load measured from the start of the element to the end of the load.
    /// Always in the X-axis of the elements local coordinates.
    pub offset_end: String,
    /// Comment for the load
    pub comment: String,
    /// The strength of the load (can be an equation). For trapezoid loads, the start and end values are separated
    /// with ; (semicolon)
    pub strength: String,
    /// The rotation of the load. 0 means the load is pointing towards positive X-axis in global coordinates (right).
    pub rotation: f64,
    /// Controls whether the load should be moved between elements in different load combinations.
    /// Only matters in load combinations that are automatically created
    pub is_moving_load: bool,
    /// The percentage of the value that is set on the 'off element' when creating the load combinations with
    /// moving loads. Default value is 0 ('off element' has no load)
    pub moving_percent: f64,
    /// Load group for the load
    pub load_group: LoadGroup,
}

impl Load {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    pub fn new_point_load(
        name: String,
        element_numbers: String,
        offset_start: String,
        strength: String,
        rotation: f64,
    ) -> Self {
        Self {
            name,
            element_numbers,
            offset_start,
            strength,
            rotation,
            load_type: Point,
            ..Self::default()
        }
    }

    pub fn new_line_load(
        name: String,
        element_numbers: String,
        offset_start: String,
        offset_end: String,
        strength: String,
        rotation: f64,
    ) -> Self {
        Self {
            name,
            element_numbers,
            offset_start,
            offset_end,
            strength,
            rotation,
            load_type: Line,
            ..Self::default()
        }
    }

    pub fn new_rotational_load(
        name: String,
        element_numbers: String,
        offset_start: String,
        strength: String,
    ) -> Self {
        Self {
            name,
            element_numbers,
            offset_start,
            strength,
            load_type: Rotational,
            ..Self::default()
        }
    }

    pub fn new_triangular_load(
        name: String,
        element_numbers: String,
        offset_start: String,
        offset_end: String,
        strength: String,
        rotation: f64,
    ) -> Self {
        Self {
            name,
            element_numbers,
            offset_start,
            offset_end,
            strength,
            rotation,
            load_type: Triangular,
            ..Self::default()
        }
    }

    /// Creates new trapezoid load. Note that the strength is separated with a semicolon ';' to
    /// beginning strength and end strength of the load
    pub fn new_trapezoid_load(
        name: String,
        element_numbers: String,
        offset_start: String,
        offset_end: String,
        strength: String,
        rotation: f64,
    ) -> Self {
        Self {
            name,
            element_numbers,
            offset_start,
            offset_end,
            strength,
            rotation,
            load_type: Trapezoid,
            ..Self::default()
        }
    }

    pub fn new_strain_load(name: String, element_numbers: String, strength: String) -> Self {
        Self {
            name,
            element_numbers,
            strength,
            load_type: Strain,
            ..Self::default()
        }
    }

    pub fn new_thermal_load(name: String, element_numbers: String, strength: String) -> Self {
        Self {
            name,
            element_numbers,
            strength,
            load_type: Thermal,
            ..Self::default()
        }
    }

    pub fn get_length(&self, equation_handler: &EquationHandler) -> f64 {
        let off_end = equation_handler
            .calculate_formula(&self.offset_end)
            .unwrap_or(0.0);
        let off_start = equation_handler
            .calculate_formula(&self.offset_start)
            .unwrap_or(0.0);
        (off_end - off_start).abs()
    }
}

impl Default for Load {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            load_type: LoadType::Point,
            element_numbers: "".to_string(),
            offset_start: "0".to_string(),
            offset_end: "L".to_string(),
            strength: "0".to_string(),
            rotation: -90.0,
            comment: "".to_string(),
            is_moving_load: false,
            moving_percent: 0.0,
            load_group: LoadGroup::PERMANENT,
        }
    }
}

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum LoadType {
    Point = 0,
    Line = 1,
    Triangular = 2,
    Rotational = 3,
    /// Note. The start and end values are separated with ; (semicolon)
    Trapezoid = 4,
    Strain = 5,
    Thermal = 6,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum CalculationLoadType {
    Point,
    Line,
    Triangular,
    Rotational,
    Strain,
}

#[derive(Debug)]
pub struct CalculationLoad {
    pub name: String,
    pub load_type: CalculationLoadType,
    pub offset_start: f64,
    pub offset_end: f64,
    pub strength: f64,
    pub rotation: f64,
    pub element_number: i32,
}

impl CalculationLoad {
    pub fn get_length(&self) -> f64 {
        (self.offset_end - self.offset_start).abs()
    }
}
