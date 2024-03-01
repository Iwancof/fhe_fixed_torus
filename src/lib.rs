#[cfg(test)]
#[macro_use]
extern crate approx;

/// Fixed point float
/// for example, 0b10000000... = 0.5
/// So, for all t in Torus, 0 <= t < 1
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Torus {
    inner: u16,
}

impl Torus {
    pub fn new(inner: u16) -> Torus {
        Torus { inner }
    }
}

impl From<f64> for Torus {
    fn from(f: f64) -> Torus {
        let inner = (f * 65536.0) as u16;
        Torus { inner }
    }
}

impl Into<f64> for Torus {
    fn into(self) -> f64 {
        (self.inner as f64) / 65536.0
    }
}

impl std::fmt::Debug for Torus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Torus({})", <Torus as Into<f64>>::into(*self))
    }
}

impl std::fmt::Display for Torus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", <Torus as Into<f64>>::into(*self))
    }
}

impl std::ops::Add for Torus {
    type Output = Torus;

    fn add(self, other: Torus) -> Torus {
        let inner = self.inner.wrapping_add(other.inner);
        Torus { inner }
    }
}

impl std::ops::Sub for Torus {
    type Output = Torus;

    fn sub(self, other: Torus) -> Torus {
        let inner = self.inner.wrapping_sub(other.inner);
        Torus { inner }
    }
}

impl std::ops::AddAssign for Torus {
    fn add_assign(&mut self, other: Torus) {
        *self = *self + other;
    }
}

impl std::ops::SubAssign for Torus {
    fn sub_assign(&mut self, other: Torus) {
        *self = *self - other;
    }
}

impl std::ops::Neg for Torus {
    type Output = Torus;

    fn neg(self) -> Torus {
        Torus::new(self.inner.wrapping_neg())
    }
}

impl std::ops::Mul<i32> for Torus {
    type Output = Torus;

    fn mul(self, rhs: i32) -> Torus {
        let inner = self.inner.wrapping_mul(rhs as u16);
        Torus { inner }
    }
}

impl std::ops::MulAssign<i32> for Torus {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl std::ops::Mul<Torus> for i32 {
    type Output = Torus;

    fn mul(self, rhs: Torus) -> Torus {
        rhs * self
    }
}

impl std::ops::Mul<f64> for Torus {
    type Output = Torus;

    fn mul(self, rhs: f64) -> Torus {
        let inner = (self.inner as f64 * rhs) as u16;
        Torus { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_new() {
        let t = Torus::new(1 << 15);
        assert_eq!(t.inner, 1 << 15);
    }

    #[test]
    fn test_from_float() {
        let t = Torus::from(0.5);
        assert_eq!(t.inner, 1 << 15);
    }

    #[test]
    fn test_into_float() {
        let t = Torus::new(1 << 15);
        let f: f64 = t.into();
        assert_eq!(f, 0.5);
    }

    #[test]
    fn test_neg() {
        let t = Torus::new(1 << 15);
        assert_eq!((-t).inner, 1 << 15);
    }

    #[test]
    fn test_neg_approx_1() {
        let t = Torus::from(0.5);
        assert_relative_eq!(<Torus as Into<f64>>::into(-t), 0.5, epsilon = 0.0001);
    }

    #[test]
    fn test_neg_approx_2() {
        let t = Torus::from(0.3);
        assert_relative_eq!(<Torus as Into<f64>>::into(-t), 0.7, epsilon = 0.0001);
    }

    #[test]
    fn test_add_zero() {
        let t1 = Torus::new(1 << 15);
        let t2 = Torus::new(1 << 15);
        let t3 = t1 + t2;
        assert_eq!(t3.inner, 0);
    }

    #[test]
    fn test_add_one() {
        let t1 = Torus::new(1 << 15);
        let t2 = Torus::new((1 << 15) + 1);
        let t3 = t1 + t2;
        assert_eq!(t3.inner, 1);
    }

    #[test]
    fn test_add_approx() {
        let f1 = 0.5;
        let f2 = 0.51;

        let t1 = Torus::from(f1);
        let t2 = Torus::from(f2);

        let t3 = t1 + t2;
        assert_relative_eq!(<Torus as Into<f64>>::into(t3), 0.01, epsilon = 0.001);
    }

    #[test]
    fn test_sub_zero() {
        let t1 = Torus::new(1 << 15);
        let t2 = Torus::new(1 << 15);
        let t3 = t1 - t2;
        assert_eq!(t3.inner, 0);
    }

    #[test]
    fn test_sub_one() {
        let t1 = Torus::new(1 << 15);
        let t2 = Torus::new((1 << 15) - 1);
        let t3 = t1 - t2;
        assert_eq!(t3.inner, 1);
    }

    #[test]
    fn test_sub_approx() {
        let f1 = 0.5;
        let f2 = 0.51;

        let t1 = Torus::from(f1);
        let t2 = Torus::from(f2);

        let t3 = t2 - t1;
        assert_relative_eq!(<Torus as Into<f64>>::into(t3), 0.01, epsilon = 0.001);
    }

    #[test]
    fn test_sub_approx_wrap() {
        let f1 = 0.5;
        let f2 = 0.51;

        let t1 = Torus::from(f1);
        let t2 = Torus::from(f2);

        let t3 = t1 - t2;
        assert_relative_eq!(<Torus as Into<f64>>::into(t3), 0.99, epsilon = 0.001);
    }

    #[test]
    fn test_add_assign() {
        let mut t1 = Torus::new(1 << 15);
        let t2 = Torus::new(1 << 15);
        t1 += t2;
        assert_eq!(t1.inner, 0);
    }

    #[test]
    fn test_sub_assign() {
        let mut t1 = Torus::new(1 << 15);
        let t2 = Torus::new(1 << 15);
        t1 -= t2;
        assert_eq!(t1.inner, 0);
    }

    #[test]
    fn test_mul() {
        let t1 = Torus::new(1 << 15);
        let t2 = t1 * 2;
        assert_eq!(t2.inner, 0);
    }

    #[test]
    fn test_mul_approx() {
        let f = 0.3;
        let t1 = Torus::from(f);
        let t2 = t1 * 2;
        assert_relative_eq!(<Torus as Into<f64>>::into(t2), 0.6, epsilon = 0.0001);
    }

    #[test]
    fn test_mul_approx_wrap() {
        let f = 0.6;
        let t1 = Torus::from(f);
        let t2 = t1 * 2;
        assert_relative_eq!(<Torus as Into<f64>>::into(t2), 0.2, epsilon = 0.0001);
    }

    #[test]
    fn test_mul_approx_neg() {
        let f = 0.3;
        let t1 = Torus::from(f);
        let t2 = t1 * -2;
        assert_relative_eq!(<Torus as Into<f64>>::into(t2), 0.4, epsilon = 0.0001);
    }

    #[test]
    fn test_mul_approx_neg_wrap() {
        let f = 0.6;
        let t1 = Torus::from(f);
        let t2 = t1 * -2;
        assert_relative_eq!(<Torus as Into<f64>>::into(t2), 0.8, epsilon = 0.0001);
    }

    #[test]
    fn test_mul_assign() {
        let mut t1 = Torus::new(1 << 15);
        t1 *= 2;
        assert_eq!(t1.inner, 0);
    }
}
