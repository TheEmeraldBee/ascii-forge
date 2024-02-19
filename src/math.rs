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

/// Creates a Vec2 from the given inputs.
pub fn vec2(x: u16, y: u16) -> Vec2 {
    Vec2 { x, y }
}
