#![allow(dead_code)]
use vputilslib::geometry2d::VpPoint;

#[derive(Debug)]
pub struct Node {
    pub point: VpPoint,
    pub support: Support,
}

impl Node {
    pub fn new(point: VpPoint, support: Support) -> Self {
        Self { point, support }
    }

    pub fn new_hinged(point: VpPoint) -> Self {
        Self{point, support: Support::new_hinged()}
    }
}

#[derive(Debug)]
pub struct Support {
    pub tx: bool,
    pub tz: bool,
    pub ry: bool,
    pub x_spring: f64,
    pub z_spring: f64,
    pub r_spring: f64,
}
impl Support {
    pub fn new() -> Self {
        Self{..Default::default()}
    }

    pub fn new_hinged() -> Self {
        Self{tx:true, tz:true, ..Default::default()}
    }
}
impl Default for Support {
    fn default() -> Self {
        Self{tx:false, tz:false, ry:false, x_spring:0.0, z_spring:0.0, r_spring:0.0}
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::node;
    use super::*;

    #[test]
    fn new() {
        let n1 = Node::new(VpPoint::new(0.0, 0.0), Support::new());
        assert_eq!(n1.point.x, 0.0);
        assert_eq!(n1.point.y, 0.0);
        assert_eq!(n1.support.tx, false);
        assert_eq!(n1.support.tz, false);
        assert_eq!(n1.support.ry, false);
        assert_eq!(n1.support.x_spring, 0.0);
        assert_eq!(n1.support.z_spring, 0.0);
        assert_eq!(n1.support.r_spring, 0.0);

        let n2 = Node::new(VpPoint::new(0.0, 0.0), Support::new_hinged());
        assert_eq!(n2.point.x, 0.0);
        assert_eq!(n2.point.y, 0.0);
        assert_eq!(n2.support.tx, true);
        assert_eq!(n2.support.tz, true);
        assert_eq!(n2.support.ry, false);
        assert_eq!(n2.support.x_spring, 0.0);
        assert_eq!(n2.support.z_spring, 0.0);
        assert_eq!(n2.support.r_spring, 0.0);

        let n3 = Node::new_hinged(VpPoint::new(0.0, 0.0));
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