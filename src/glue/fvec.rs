use bevy::prelude::*;
use fixed::traits::ToFixed;

use super::Fx;

#[macro_export]
macro_rules! fx {
    ($e:expr) => {
        Fx::from_num($e)
    };
}
pub use fx;

#[derive(Clone, Copy, Debug, Default)]
pub struct FVec2 {
    pub x: Fx,
    pub y: Fx,
}
impl FVec2 {
    pub const ZERO: Self = Self::const_new(Fx::ZERO, Fx::ZERO);
    pub const ONE: Self = Self::const_new(Fx::ONE, Fx::ONE);
    pub const X: Self = Self::const_new(Fx::ONE, Fx::ZERO);
    pub const Y: Self = Self::const_new(Fx::ZERO, Fx::ONE);

    pub fn new<X: ToFixed, Y: ToFixed>(x: X, y: Y) -> Self {
        Self {
            x: fx!(x),
            y: fx!(y),
        }
    }
    pub const fn const_new(x: Fx, y: Fx) -> Self {
        Self { x, y }
    }
    pub fn round(&self) -> IVec2 {
        IVec2 {
            x: self.x.round().to_num(),
            y: self.y.round().to_num(),
        }
    }
    pub fn length_squared(&self) -> Fx {
        self.x * self.x + self.y * self.y
    }
    pub fn length(&self) -> Fx {
        self.length_squared().sqrt()
    }
    pub fn dot(&self, o: Self) -> Fx {
        self.x * o.x + self.y * o.y
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x.to_num(), self.y.to_num())
    }
}
impl PartialEq for FVec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for FVec2 {}
impl std::ops::Add for FVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::AddAssign for FVec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl std::ops::Sub for FVec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl std::ops::SubAssign for FVec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<Fx> for FVec2 {
    type Output = Self;

    fn mul(self, rhs: Fx) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl std::ops::MulAssign<Fx> for FVec2 {
    fn mul_assign(&mut self, rhs: Fx) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl std::ops::Mul<FVec2> for Fx {
    type Output = FVec2;

    fn mul(self, rhs: FVec2) -> Self::Output {
        FVec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Div<Fx> for FVec2 {
    type Output = Self;

    fn div(self, rhs: Fx) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl std::ops::DivAssign<Fx> for FVec2 {
    fn div_assign(&mut self, rhs: Fx) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
impl std::ops::Div<FVec2> for Fx {
    type Output = FVec2;

    fn div(self, rhs: FVec2) -> Self::Output {
        FVec2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}
