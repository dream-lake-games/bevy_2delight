use bevy::prelude::*;
use fixed::types::I32F32;

type F = I32F32;

#[derive(Clone, Copy, Debug)]
pub struct Frac {
    num: F,
}
impl Default for Frac {
    fn default() -> Self {
        Self { num: F::ZERO }
    }
}
impl Frac {
    pub const ZERO: Self = Self::const_whole(0);
    pub const ONE: Self = Self::const_whole(1);
    pub const TWO: Self = Self::const_whole(2);

    pub fn new(whole: i32, cent: i8) -> Self {
        Self {
            num: if whole >= 0 {
                F::from_num(whole) + F::from_num(cent) / 100
            } else {
                F::from_num(whole) - F::from_num(cent) / 100
            },
        }
    }
    pub fn whole(whole: i32) -> Self {
        Self {
            num: F::from_num(whole),
        }
    }
    pub const fn const_whole(whole: i32) -> Self {
        Self {
            num: F::const_from_int(whole as i64),
        }
    }
    pub fn cent(cent: i8) -> Self {
        Self {
            num: F::from_num(cent) / 100,
        }
    }
    pub const fn const_cent(cent: i8) -> Self {
        Self {
            num: F::const_from_int(cent as i64 / 100),
        }
    }
    pub fn with_cent(mut self, cent: i8) -> Self {
        if self.num >= 0 {
            self.num += F::from_num(cent) / 100;
        } else {
            self.num -= F::from_num(cent) / 100;
        }
        self
    }
    pub fn with_mil(mut self, mil: i16) -> Self {
        if self.num >= 0 {
            self.num += F::from_num(mil) / 1_000;
        } else {
            self.num -= F::from_num(mil) / 1_000;
        }
        self
    }
    pub fn with_micro(mut self, micro: i32) -> Self {
        if self.num >= 0 {
            self.num += F::from_num(micro) / 1_000_000;
        } else {
            self.num -= F::from_num(micro) / 1_000_000;
        }
        self
    }

    pub fn round(&self) -> i32 {
        self.num.round().to_num()
    }
    pub fn abs(&self) -> Frac {
        Frac {
            num: self.num.abs(),
        }
    }
    pub fn squared(&self) -> Frac {
        Frac {
            num: (self.num * self.num),
        }
    }
    pub fn signum(&self) -> Frac {
        if self.num < 0 {
            Frac::whole(-1)
        } else {
            Frac::whole(1)
        }
    }
    pub fn rem_euclid(&self, other: Frac) -> Frac {
        Self {
            num: self.num.rem_euclid(other.num),
        }
    }

    pub fn as_f32(&self) -> f32 {
        self.num.to_num()
    }
    pub fn as_micros(&self) -> i64 {
        (self.num * 1_000_000).round().to_num()
    }
}
impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num
    }
}
impl Eq for Frac {}
impl std::ops::Add for Frac {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num: self.num + rhs.num,
        }
    }
}
impl std::ops::AddAssign for Frac {
    fn add_assign(&mut self, rhs: Self) {
        self.num += rhs.num;
    }
}
impl std::ops::Sub for Frac {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            num: self.num - rhs.num,
        }
    }
}
impl std::ops::SubAssign for Frac {
    fn sub_assign(&mut self, rhs: Self) {
        self.num -= rhs.num;
    }
}
impl std::ops::Mul for Frac {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            num: self.num * rhs.num,
        }
    }
}
impl std::ops::MulAssign for Frac {
    fn mul_assign(&mut self, rhs: Self) {
        self.num = self.num * rhs.num;
    }
}
impl std::ops::Div for Frac {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            num: self.num / rhs.num,
        }
    }
}
impl std::ops::DivAssign for Frac {
    fn div_assign(&mut self, rhs: Self) {
        self.num = self.num / rhs.num;
    }
}
impl std::ops::Neg for Frac {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.num = -self.num;
        self
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num.partial_cmp(&other.num)
    }
}
impl Ord for Frac {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num.cmp(&other.num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Frac::new(1, 48);
        let b = Frac::new(2, 55);
        let mut c = a + b;
        assert_eq!(c, Frac::new(4, 3));
        c += b;
        assert_eq!(c, Frac::new(6, 58));
    }

    #[test]
    fn test_sub() {
        let a = Frac::new(1, 48);
        let b = Frac::new(2, 55);
        let mut c = a - b;
        assert_eq!(c, Frac::new(-1, 7));
        c -= b;
        assert_eq!(c, Frac::new(-3, 62));
    }

    #[test]
    fn test_mul() {
        let a = Frac::new(-1, 48);
        let b = Frac::new(2, 55);
        let mut c = a * b;
        assert_eq!(c, Frac::whole(-3).with_mil(774));
        c *= b;
        assert_eq!(c, Frac::whole(-9).with_micro(623_700));
    }

    #[test]
    fn test_div() {
        let a = Frac::whole(-1);
        let b = Frac::whole(2);
        let mut c = a / b;
        assert_eq!(c, Frac::ZERO.with_cent(-50));
        c /= Frac::new(-4, 30);
        assert_eq!(c, Frac::ZERO.with_micro(116_279));
    }

    #[test]
    fn test_round() {
        let a = Frac::new(-1, 60).round();
        assert_eq!(a, -2);
        let b = Frac::new(-1, 30).round();
        assert_eq!(b, -1);
        let c = Frac::new(1, 30).round();
        assert_eq!(c, 1);
        let d = Frac::new(1, 60).round();
        assert_eq!(d, 2);
    }

    #[test]
    fn test_square() {
        let a = Frac::whole(7);
        assert_eq!(a.squared(), Frac::whole(49));
    }

    #[test]
    fn test_signum() {
        let a = Frac::whole(-1);
        let b = Frac::ZERO;
        let c = Frac::whole(1);
        assert_eq!(a.signum(), Frac::whole(-1));
        assert_eq!(b.signum(), Frac::whole(1));
        assert_eq!(c.signum(), Frac::whole(1));
    }
}
