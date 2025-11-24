use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: u32,
    pub y: u32,
}

impl Vec2 {
    pub fn zero() -> Vec2 {
        Vec2 { x: 0, y: 0 }
    }

    pub fn from_signed_tuple(t: (u32, u32)) -> Vec2 {
        Vec2 { x: t.0, y: t.1 }
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
