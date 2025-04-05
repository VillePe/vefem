use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize)]
pub struct InternalForceResults {
    /// The element number to which the internal forces are linked
    pub element_number: i32,
    pub axial_forces: Vec<InternalForcePoint>,
    pub shear_forces: Vec<InternalForcePoint>,
    pub moment_forces: Vec<InternalForcePoint>,
    pub deflections: Vec<InternalForcePoint>
}

impl InternalForceResults {
    pub fn get_force_at(&self, force_type: ForceType, pos_on_element: f64) -> Option<InternalForcePoint> {
        match force_type {
            ForceType::Axial => get_force_at_interpolated(&self.axial_forces, pos_on_element),
            ForceType::Shear => get_force_at_interpolated(&self.shear_forces, pos_on_element),
            ForceType::Moment => get_force_at_interpolated(&self.moment_forces, pos_on_element),
            ForceType::Deflection => get_force_at_interpolated(&self.deflections, pos_on_element),
        }
    }


    pub fn get_force_at_exact(&self, force_type: ForceType, pos_on_element: f64) -> Option<&InternalForcePoint> {
        match force_type {
            ForceType::Axial => self.axial_forces.iter().find(|f| f.pos_on_element >= pos_on_element),
            ForceType::Shear => self.shear_forces.iter().find(|f| f.pos_on_element >= pos_on_element),
            ForceType::Moment => self.moment_forces.iter().find(|f| f.pos_on_element >= pos_on_element),
            ForceType::Deflection => self.deflections.iter().find(|f| f.pos_on_element >= pos_on_element),
        }
    }
}

pub fn get_force_at_interpolated(points: &Vec<InternalForcePoint>, pos_on_element: f64) -> Option<InternalForcePoint> {
    if points.len() <= 0 {
        return None;
    }
    let mut prev_point: Option<&InternalForcePoint> = None;
    for p in points {
        if p.pos_on_element == pos_on_element {
           return Some(p.clone()); 
        }
        if p.pos_on_element > pos_on_element {
            // If there is no previous point, the current point 
            let (prev_pos, prev_val_x, prev_val_y) = if prev_point.is_none() {
                (0.0, 0.0, 0.0)
            } else {
                let prev = prev_point.unwrap();
                (prev.pos_on_element, prev.value_x, prev.value_y)
            };
            let next_pos = p.pos_on_element;                    
            let interp_val_x = prev_val_x + (p.value_x - prev_val_x) * (pos_on_element - prev_pos) / (next_pos - prev_pos);
            let interp_val_y = prev_val_y + (p.value_y - prev_val_y) * (pos_on_element - prev_pos) / (next_pos - prev_pos);
            return Some(InternalForcePoint{
                force_type: p.force_type,
                value_x: interp_val_x,
                value_y: interp_val_y,
                pos_on_element: pos_on_element,
                element_number: p.element_number,
                load_comb_number: p.load_comb_number
            })
        }
        prev_point = Some(p);
    }
    match prev_point {
        Some(prev) => {
            return Some(prev.clone())
        } 
        None => return None
    }    
}

#[derive(Copy, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ForceType {
    Axial = 0,
    Shear = 1,
    Moment = 2,
    Deflection = 3
}

#[cfg(test)]
mod test {
    use crate::results::{internal_force_results::get_force_at_interpolated, ForceType, InternalForcePoint};

    #[test]
    fn test_internal_force_point_interpolation() {
        let points = vec![InternalForcePoint{
                pos_on_element: 10.0, 
                value_x: 5.0, value_y: 10.0, 
                force_type: ForceType::Deflection, element_number: 1, load_comb_number: 1
            },
            InternalForcePoint{
                pos_on_element: 20.0, 
                value_x: 15.0, value_y: 50.0, 
                force_type: ForceType::Deflection, element_number: 1, load_comb_number: 1
            }];
        let interp = get_force_at_interpolated(&points, 7.0);
        assert_eq!(interp.unwrap().value_x, 3.5);
        assert_eq!(interp.unwrap().value_y, 7.0);

        let interp = get_force_at_interpolated(&points, 14.0);
        assert_eq!(interp.unwrap().value_x, 9.0);
        assert_eq!(interp.unwrap().value_y, 26.0);
    }
}