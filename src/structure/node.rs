#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use vputilslib::geometry2d::VpPoint;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub number: i32,
    pub point: VpPoint,
    pub support: Support,
}

impl Node {
    /// Creates new node (support lock values are all free)
    pub fn new(number: i32, point: VpPoint, support: Support) -> Self {
        Self {
            number,
            point,
            support,
        }
    }

    /// Creates new node that has the free support lock values (translations not locked, rotation not locked)
    pub fn new_free(number: i32, point: VpPoint) -> Self {
        Self {
            number,
            point,
            support: Support::new(),
        }
    }

    /// Creates new node that has the hinged support lock values (translations locked, rotation not locked)
    pub fn new_hinged(number: i32, point: VpPoint) -> Self {
        Self {
            number,
            point,
            support: Support::new_hinged(),
        }
    }

    /// Creates new node that has the fixed support lock values (translations locked, rotation locked)
    pub fn new_fixed(number: i32, point: VpPoint) -> Self {
        Self {
            number,
            point,
            support: Support::new_fixed(),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let n1 = Node::new(1, VpPoint::new(0.0, 0.0), Support::new());
        assert_eq!(n1.point.x, 0.0);
        assert_eq!(n1.point.y, 0.0);
        assert_eq!(n1.support.tx, false);
        assert_eq!(n1.support.tz, false);
        assert_eq!(n1.support.ry, false);
        assert_eq!(n1.support.x_spring, 0.0);
        assert_eq!(n1.support.z_spring, 0.0);
        assert_eq!(n1.support.r_spring, 0.0);

        let n2 = Node::new(2, VpPoint::new(0.0, 0.0), Support::new_hinged());
        assert_eq!(n2.point.x, 0.0);
        assert_eq!(n2.point.y, 0.0);
        assert_eq!(n2.support.tx, true);
        assert_eq!(n2.support.tz, true);
        assert_eq!(n2.support.ry, false);
        assert_eq!(n2.support.x_spring, 0.0);
        assert_eq!(n2.support.z_spring, 0.0);
        assert_eq!(n2.support.r_spring, 0.0);

        let n3 = Node::new_hinged(3, VpPoint::new(0.0, 0.0));
        assert_eq!(n3.point.x, 0.0);
        assert_eq!(n3.point.y, 0.0);
        assert_eq!(n3.support.tx, true);
        assert_eq!(n3.support.tz, true);
        assert_eq!(n3.support.ry, false);
        assert_eq!(n3.support.x_spring, 0.0);
        assert_eq!(n3.support.z_spring, 0.0);
        assert_eq!(n3.support.r_spring, 0.0);
    }
}
