use std::{fmt::{Display, Formatter}, cmp::PartialEq};
use std::ops::{Add, Sub};


#[derive(Debug)]
pub struct Vec2d {
    x : f32,
    y : f32,
}

pub trait Vec2dMath {
    fn scalar_prod(v1: &Vec2d, v2: &Vec2d) -> f32;
}

impl Vec2d {
    pub fn new(x : f32, y : f32) -> Vec2d {
        Vec2d { x, y }
    }

    pub fn get_unit_vec() -> Vec2d {
        Vec2d { x: 1.0, y: 0.0 }
    }

    pub fn norm(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn scale(&mut self, factor: f32) {
        self.x *= factor;
        self.y *= factor;
    }

    pub fn scalar_prod(&self, other: &Vec2d) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Vec2dMath for Vec2d {
    fn scalar_prod(v1: &Vec2d, v2: &Vec2d) -> f32 {
        v1.x * v2.x + v1.y * v2.y
    }
}

impl Display for Vec2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec({}, {})", self.x, self.y)
    }
}

impl PartialEq for Vec2d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<'a, 'b> Add<&'b Vec2d> for &'a Vec2d {
    type Output = Vec2d; // Output describes the resulting type after applying the + operator.
                        // Self describes the current type, here it means Vec2d

    fn add(self, other: &'b Vec2d) -> Vec2d {
        Vec2d::new(self.x + other.x, self.y + other.y)
    }
}

impl<'a, 'b> Sub<&'b Vec2d> for &'a Vec2d {
    type Output = Vec2d;

    fn sub(self, other: &'b Vec2d) -> Vec2d {
        Vec2d::new(self.x - other.x, self.y - other.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const V1: Vec2d = Vec2d { x: 3.0, y: 4.0};
    const V2: Vec2d = Vec2d { x: 5.0, y: 12.0};
    const V3: Vec2d = Vec2d { x: 8.0, y: 15.0};

    #[test]
    fn test_get_unit_vec() {
        assert_eq!(Vec2d::get_unit_vec(), Vec2d{x: 1.0, y: 0.0});
    }

    #[test]
    fn test_norm() {
        assert_eq!(5.0, V1.norm());
        assert_eq!(13.0, V2.norm());
        assert_eq!(17.0, V3.norm());
    }

    #[test]
    fn test_scale() {
        let mut v1 = Vec2d { x: 2.0, y: 3.0};
        let mut v2 = Vec2d { x: 1.0, y: 0.0};
        let mut v3 = Vec2d { x: -2.0, y: -1.0};

        v1.scale(3.0);
        v2.scale(0.5);
        v3.scale(-1.5);

        assert_eq!(Vec2d { x: 6.0, y: 9.0}, v1);
        assert_eq!(Vec2d { x: 0.5, y: 0.0}, v2);
        assert_eq!(Vec2d { x: 3.0, y: 1.5}, v3);
    }

    #[test]
    fn test_scalar_prod() {
        assert_eq!(V1.scalar_prod(&V2), 63.0);
        assert_eq!(V2.scalar_prod(&V3), 220.0);
    }

    #[test]
    fn test_add() {
        let v1 = Vec2d { x: 1.0, y: 2.0};
        let v2 = Vec2d { x: 3.0, y: 4.0};
        let v3 = Vec2d { x: 4.0, y: 6.0};

        assert_eq!(&v1 + &v2, v3);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec2d { x: 5.0, y: 6.0};
        let v2 = Vec2d { x: 3.0, y: 4.0};
        let v3 = Vec2d { x: 2.0, y: 2.0};

        assert_eq!(&v1 - &v2, v3);
    }
}
