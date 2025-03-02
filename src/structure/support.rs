use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Support {
    /// If set to true, the translation in the global X-axis is locked at current node
    pub tx: bool,
    /// If set to true, the translation in the global Z-axis is locked at current node
    pub tz: bool,
    /// If set to true, the translation about the global Y-axis is locked at current node
    pub ry: bool,
    /// The spring constant in global X-axis
    pub x_spring: f64,
    /// The spring constant in global Y-axis
    pub z_spring: f64,
    /// The spring constant about global Y-axis
    pub r_spring: f64,
}
impl Support {
    /// Creates new support that has no locks set (translations and rotation are not locked)
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Creates new support that is hinged (translations locked, rotation not locked)
    pub fn new_hinged() -> Self {
        Self {
            tx: true,
            tz: true,
            ..Default::default()
        }
    }

    /// Creates new support that is hinged (translations locked, rotation not locked)
    pub fn new_fixed() -> Self {
        Self {
            tx: true,
            tz: true,
            ry: true,
            ..Default::default()
        }
    }

    /// Returns true if the support translation is free at given index (0=tx, 1=tz, 2=ry)
    pub fn get_support_lock(&self, index: usize) -> bool {
        match index {
            0 => self.tx,
            1 => self.tz,
            2 => self.ry,
            _ => panic!(
                "Tried to get degree of freedom from support outside of degrees of freedom count!"
            ),
        }
    }

    /// Returns the spring value at given index (0=x, 1=z, 2=r)
    pub fn get_support_spring(&self, index: usize) -> f64 {
        match index {
            0 => self.x_spring,
            1 => self.z_spring,
            2 => self.r_spring,
            _ => panic!(
                "Tried to get spring value from support outside of degrees of freedom count!"
            ),
        }
    }
}
impl Default for Support {
    fn default() -> Self {
        Self {
            tx: false,
            tz: false,
            ry: false,
            x_spring: 0.0,
            z_spring: 0.0,
            r_spring: 0.0,
        }
    }
}