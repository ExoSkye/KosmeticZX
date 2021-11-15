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