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

pub fn vec2(x: u16, y: u16) -> Vec2 {
    Vec2 { x, y }
}
