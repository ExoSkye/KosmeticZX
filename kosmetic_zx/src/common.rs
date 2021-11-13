use std::fmt::Debug;
use derive_more::*;
use sdl2::rect::Point;

pub use u16 as Address;
pub use u8 as Byte;

#[derive(PartialEq, Eq, Hash, Into, Add, Mul, Not, Sum, AddAssign, MulAssign, Constructor, Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: u16,
    pub y: u16
}

#[derive(PartialEq, Eq, Hash, Into, Add, Mul, Not, Sum, AddAssign, MulAssign, Constructor, Copy, Clone, Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16
}

impl Rect {
    pub fn inside(&self, other: Rect) -> bool {
        return if other.x > self.x &&
            other.y > self.y &&
            (other.x + other.w) < (self.x + self.w) &&
            (other.y + other.h) < (self.y + self.h) {
            true
        } else {
            false
        }
    }
    pub fn vec_inside(&self, other: Vec2) -> bool {
        return self.inside(Rect::new(other.x, other.y, other.x, other.y))
    }
}

impl Vec2 {
    pub fn inside(self, rect: Rect) -> bool {
        return rect.vec_inside(self);
    }
}

impl Into<Point> for Vec2 {
    fn into(self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

impl Into<sdl2::rect::Rect> for Rect {
    fn into(self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}

impl Into<Point> for Rect {
    fn into(self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

impl Into<sdl2::rect::Rect> for Vec2 {
    fn into(self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x as i32, self.y as i32, self.x as u32, self.y as u32)
    }
}