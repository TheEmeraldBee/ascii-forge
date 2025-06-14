use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A 2d Vector that has no math, is only used as a pretty version of a tuple of u16s
/// Can be made from (u16, u16).
/// Using a single u16.into() will create a vec2 where both values are the same.
#[derive(Default, Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl From<(u16, u16)> for Vec2 {
    fn from(value: (u16, u16)) -> Self {
        vec2(value.0, value.1)
    }
}

impl From<u16> for Vec2 {
    fn from(value: u16) -> Self {
        vec2(value, value)
    }
}

impl<V: Into<Vec2>> Add<V> for Vec2 {
    type Output = Vec2;
    fn add(mut self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl<V: Into<Vec2>> AddAssign<V> for Vec2 {
    fn add_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<V: Into<Vec2>> Sub<V> for Vec2 {
    type Output = Vec2;
    fn sub(mut self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl<V: Into<Vec2>> SubAssign<V> for Vec2 {
    fn sub_assign(&mut self, rhs: V) {
        let rhs = rhs.into();
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// Creates a Vec2 from the given inputs.
pub fn vec2(x: u16, y: u16) -> Vec2 {
    Vec2 { x, y }
}
