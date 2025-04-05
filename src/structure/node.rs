#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use vputilslib::geometry2d::VpPoint;

use super::Support;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
