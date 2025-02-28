use bevy::prelude::*;

use super::frac::Frac;

#[derive(Clone, Copy, Debug, Default, Reflect)]
pub struct FVec2 {
    pub x: Frac,
    pub y: Frac,
}
impl FVec2 {
    pub const ZERO: Self = Self::const_new(Frac::ZERO, Frac::ZERO);
    pub const ONE: Self = Self::const_new(Frac::ONE, Frac::ONE);
    pub const X: Self = Self::const_new(Frac::ONE, Frac::ZERO);
    pub const Y: Self = Self::const_new(Frac::ZERO, Frac::ONE);

    pub fn new(x: Frac, y: Frac) -> Self {
        Self { x, y }
    }
    pub const fn const_new(x: Frac, y: Frac) -> Self {
        Self { x, y }
    }
    pub fn round(&self) -> IVec2 {
        IVec2 {
            x: self.x.round(),
            y: self.y.round(),
        }
    }
    pub fn length_squared(&self) -> Frac {
        self.x.squared() + self.y.squared()
    }
    pub fn dot(&self, o: Self) -> Frac {
        self.x * o.x + self.y * o.y
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x.as_f32(), self.y.as_f32())
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

impl std::ops::Mul<Frac> for FVec2 {
    type Output = Self;

    fn mul(self, rhs: Frac) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl std::ops::MulAssign<Frac> for FVec2 {
    fn mul_assign(&mut self, rhs: Frac) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl std::ops::Mul<FVec2> for Frac {
    type Output = FVec2;

    fn mul(self, rhs: FVec2) -> Self::Output {
        FVec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Div<Frac> for FVec2 {
    type Output = Self;

    fn div(self, rhs: Frac) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl std::ops::DivAssign<Frac> for FVec2 {
    fn div_assign(&mut self, rhs: Frac) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
impl std::ops::Div<FVec2> for Frac {
    type Output = FVec2;

    fn div(self, rhs: FVec2) -> Self::Output {
        FVec2 {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul() {
        let mut a = FVec2::new(Frac::new(1, 11), Frac::new(-1, 11));
        let b = a * Frac::whole(3);
        let c = Frac::whole(4) * a;
        a *= Frac::whole(5);
        assert_eq!(a, FVec2::new(Frac::new(5, 55), Frac::new(-5, 55)));
        assert_eq!(b, FVec2::new(Frac::new(3, 33), Frac::new(-3, 33)));
        assert_eq!(c, FVec2::new(Frac::new(4, 44), Frac::new(-4, 44)));
    }

    #[test]
    fn div() {
        let a = FVec2::new(Frac::new(2, 22), Frac::new(3, 33));
        let b = a / Frac::new(1, 11);
        assert_eq!(b, FVec2::new(Frac::whole(2), Frac::whole(3)));
    }
}
