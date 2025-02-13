use bevy::prelude::*;

const DENOM: i64 = 1_000_000;

#[derive(Clone, Copy, Debug, Reflect)]
pub struct Frac {
    num: i64,
}
impl Default for Frac {
    fn default() -> Self {
        Self { num: 0 }
    }
}
impl Frac {
    pub const ZERO: Self = Self::const_whole(0);
    pub const ONE: Self = Self::const_whole(1);

    pub fn new(whole: i32, cent: i8) -> Self {
        Self {
            num: if whole >= 0 {
                (whole as i64) * DENOM + (cent as i64) * (DENOM / 100)
            } else {
                (whole as i64) * DENOM - (cent as i64) * (DENOM / 100)
            },
        }
    }
    pub fn whole(whole: i32) -> Self {
        Self {
            num: (whole as i64) * DENOM,
        }
    }
    pub const fn const_whole(whole: i32) -> Self {
        Self {
            num: (whole as i64) * DENOM,
        }
    }
    pub fn cent(cent: i8) -> Self {
        Self {
            num: cent as i64 * DENOM / 100,
        }
    }
    pub fn with_cent(mut self, cent: i8) -> Self {
        if self.num >= 0 {
            self.num += (cent as i64) * (DENOM / 100)
        } else {
            self.num -= (cent as i64) * (DENOM / 100)
        }
        self
    }
    pub fn with_mil(mut self, mil: i16) -> Self {
        if self.num >= 0 {
            self.num += (mil as i64) * (DENOM / 1_000)
        } else {
            self.num -= (mil as i64) * (DENOM / 1_000)
        }
        self
    }
    pub fn with_micro(mut self, micro: i32) -> Self {
        if self.num >= 0 {
            self.num += (micro as i64) * (DENOM / 1_000_000)
        } else {
            self.num -= (micro as i64) * (DENOM / 1_000_000)
        }
        self
    }

    pub fn round(&self) -> i32 {
        let div = (self.num / DENOM) as i32;
        let rem = (self.num % DENOM) as i32;
        let half_denom = DENOM as i32 / 2;
        println!("d r {div} {rem}");
        if self.num > 0 {
            if rem >= half_denom {
                div + 1
            } else {
                div
            }
        } else {
            if rem <= -half_denom {
                div - 1
            } else {
                div
            }
        }
    }
    pub fn abs(&self) -> Frac {
        Frac {
            num: self.num.abs(),
        }
    }
    pub fn squared(&self) -> Frac {
        Frac {
            num: (self.num * self.num) / DENOM,
        }
    }
    pub fn signum(&self) -> Frac {
        if self.num < 0 {
            Frac::whole(-1)
        } else {
            Frac::whole(1)
        }
    }

    pub fn as_f32(&self) -> f32 {
        self.num as f32 / DENOM as f32
    }
    pub fn as_micros(&self) -> i64 {
        self.num
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
            num: (self.num * rhs.num) / DENOM,
        }
    }
}
impl std::ops::MulAssign for Frac {
    fn mul_assign(&mut self, rhs: Self) {
        self.num = (self.num * rhs.num) / DENOM;
    }
}
impl std::ops::Div for Frac {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            num: (self.num * DENOM) / rhs.num,
        }
    }
}
impl std::ops::DivAssign for Frac {
    fn div_assign(&mut self, rhs: Self) {
        self.num = (self.num * DENOM) / rhs.num;
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
