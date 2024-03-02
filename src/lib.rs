#[cfg(test)]
#[macro_use]
extern crate approx;

type TorusRepr = u32;

/// Fixed point float
/// for example, 0b10000000... = 0.5
/// So, for all t in Torus, 0 <= t < 1
#[derive(Clone, Copy)]
pub struct Torus {
    inner: TorusRepr,
}

impl Torus {
    const SHIFT: u32 = TorusRepr::MAX;

    pub fn new(inner: TorusRepr) -> Torus {
        Torus { inner }
    }

    pub fn sign(&self) -> i32 {
        if self.inner < (Torus::SHIFT / 2) {
            1
        } else {
            -1
        }
    }

    #[cfg(feature = "random")]
    pub fn normal(std: f64, state: &mut impl rand::Rng) -> Torus {
        use rand::distributions::Distribution;

        let normal = statrs::distribution::Normal::new(0., std).unwrap();
        let sample = normal.sample(state);
        // TODO: instead of generating a float and converting it to integer, generate an integer directly

        Torus::from(sample)
    }

    #[cfg(feature = "random")]
    pub fn uniform(state: &mut impl rand::Rng) -> Torus {
        use rand::distributions::Distribution;

        let uniform = rand::distributions::Uniform::new(0., 1.);
        let sample = uniform.sample(state);
        // TODO: instead of generating a float and converting it to integer, generate an integer directly

        Torus::from(sample)
    }
}

impl num_traits::identities::ConstZero for Torus {
    const ZERO: Self = Torus { inner: 0 };
}

impl num_traits::identities::Zero for Torus {
    fn zero() -> Self {
        Self { inner: 0 }
    }

    fn is_zero(&self) -> bool {
        self.inner == 0
    }
}

impl From<f64> for Torus {
    fn from(f: f64) -> Torus {
        let f = f.rem_euclid(1.0);
        let inner = (f * (Torus::SHIFT as f64)) as TorusRepr;
        Torus { inner }
    }
}

impl From<Torus> for f64 {
    fn from(t: Torus) -> f64 {
        // TODO: overflow?
        (t.inner as f64) / (Torus::SHIFT as f64)
    }
}


impl std::fmt::Debug for Torus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Torus({})", f64::from(*self))
    }
}

impl std::fmt::Display for Torus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", f64::from(*self))
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
        let inner = self.inner.wrapping_mul(rhs as TorusRepr);
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
        let inner = (self.inner as f64 * rhs) as TorusRepr;
        Torus { inner }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZERO_POINTS_FIVE: TorusRepr = 1 << (TorusRepr::BITS - 1);

    #[test]
    fn test_sign_plus() {
        let t = Torus::from(0.4);
        assert_eq!(t.sign(), 1);
    }
    
    #[test]
    fn test_sign_minus() {
        let t = Torus::from(0.6);
        assert_eq!(t.sign(), -1);
    }

    #[test]
    fn test_from_float() {
        let t = Torus::from(0.5);
        assert_relative_eq!(f64::from(t), 0.5, epsilon = 0.0001);
    }

    #[test]
    fn test_from_float_wrap_1() {
        let t = Torus::from(1.5);
        assert_relative_eq!(f64::from(t), 0.5, epsilon = 0.0001);
    }

    #[test]
    fn test_from_float_wrap_2() {
        let t = Torus::from(134.2);
        assert_relative_eq!(f64::from(t), 0.2, epsilon = 0.0001);
    }

    #[test]
    fn test_from_float_neg() {
        let t = Torus::from(-0.5);
        assert_relative_eq!(f64::from(t), 0.5, epsilon = 0.0001);
    }

    #[test]
    fn test_from_float_neg_wrap() {
        let t = Torus::from(-1.5);
        assert_relative_eq!(f64::from(t), 0.5, epsilon = 0.0001);
    }

    #[test]
    fn test_from_float_neg_wrap_2() {
        let t = Torus::from(-134.2);
        assert_relative_eq!(f64::from(t), 0.8, epsilon = 0.0001);
    }

    #[test]
    fn test_into_float() {
        let t = Torus::new(ZERO_POINTS_FIVE);
        let f: f64 = t.into();
        assert_relative_eq!(f, 0.5, epsilon = 0.0001);
    }

    #[test]
    fn test_neg() {
        let t = Torus::new(ZERO_POINTS_FIVE);
        assert_relative_eq!(f64::from(-t), 0.5, epsilon = 0.0001);
    }

    #[test]
    fn test_neg_approx_1() {
        let t = Torus::from(0.5);
        assert_relative_eq!(f64::from(-t), 0.5, epsilon = 0.0001);
    }

    #[test]
    fn test_neg_approx_2() {
        let t = Torus::from(0.3);
        assert_relative_eq!(f64::from(-t), 0.7, epsilon = 0.0001);
    }

    #[test]
    fn test_add_zero() {
        let t1 = Torus::new(ZERO_POINTS_FIVE);
        let t2 = Torus::new(ZERO_POINTS_FIVE);
        let t3 = t1 + t2;
        assert_relative_eq!(f64::from(t3), 0.0, epsilon = 0.0001);
    }

    #[test]
    fn test_add_one() {
        let t1 = Torus::new(ZERO_POINTS_FIVE);
        let t2 = Torus::new(ZERO_POINTS_FIVE + 1);
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
        assert_relative_eq!(f64::from(t3), 0.01, epsilon = 0.001);
    }

    #[test]
    fn test_sub_zero() {
        let t1 = Torus::new(ZERO_POINTS_FIVE);
        let t2 = Torus::new(ZERO_POINTS_FIVE);
        let t3 = t1 - t2;
        assert_eq!(t3.inner, 0);
    }

    #[test]
    fn test_sub_one() {
        let t1 = Torus::new(ZERO_POINTS_FIVE);
        let t2 = Torus::new(ZERO_POINTS_FIVE - 1);
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
        assert_relative_eq!(f64::from(t3), 0.01, epsilon = 0.001);
    }

    #[test]
    fn test_sub_approx_wrap() {
        let f1 = 0.5;
        let f2 = 0.51;

        let t1 = Torus::from(f1);
        let t2 = Torus::from(f2);

        let t3 = t1 - t2;
        assert_relative_eq!(f64::from(t3), 0.99, epsilon = 0.001);
    }

    #[test]
    fn test_add_assign() {
        let mut t1 = Torus::new(ZERO_POINTS_FIVE);
        let t2 = Torus::new(ZERO_POINTS_FIVE);
        t1 += t2;
        assert_eq!(t1.inner, 0);
    }

    #[test]
    fn test_sub_assign() {
        let mut t1 = Torus::new(ZERO_POINTS_FIVE);
        let t2 = Torus::new(ZERO_POINTS_FIVE);
        t1 -= t2;
        assert_eq!(t1.inner, 0);
    }

    #[test]
    fn test_mul() {
        let t1 = Torus::new(ZERO_POINTS_FIVE);
        let t2 = t1 * 2;
        assert_eq!(t2.inner, 0);
    }

    #[test]
    fn test_mul_approx() {
        let f = 0.3;
        let t1 = Torus::from(f);
        let t2 = t1 * 2;
        assert_relative_eq!(f64::from(t2), 0.6, epsilon = 0.0001);
    }

    #[test]
    fn test_mul_approx_wrap() {
        let f = 0.6;
        let t1 = Torus::from(f);
        let t2 = t1 * 2;
        assert_relative_eq!(f64::from(t2), 0.2, epsilon = 0.0001);
    }

    #[test]
    fn test_mul_approx_neg() {
        let f = 0.3;
        let t1 = Torus::from(f);
        let t2 = t1 * -2;
        assert_relative_eq!(f64::from(t2), 0.4, epsilon = 0.0001);
    }

    #[test]
    fn test_mul_approx_neg_wrap() {
        let f = 0.6;
        let t1 = Torus::from(f);
        let t2 = t1 * -2;
        assert_relative_eq!(f64::from(t2), 0.8, epsilon = 0.0001);
    }

    #[test]
    fn test_mul_assign() {
        let mut t1 = Torus::new(ZERO_POINTS_FIVE);
        t1 *= 2;
        assert_eq!(t1.inner, 0);
    }

    #[cfg(feature = "random")]
    #[test]
    fn test_normal() {
        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            let t = Torus::normal(0.1, &mut rng);
            assert!(f64::from(t) >= 0.0);
            assert!(f64::from(t) < 1.0);
        }
    }

    #[cfg(feature = "random")]
    #[test]
    fn test_normal_approx() {
        let sum: f64 = (0..1000)
            .map(|_| {
                let mut rng = rand::thread_rng();
                let t = Torus::normal(0.1, &mut rng);
                f64::from(t)
            })
            .sum();

        assert_relative_eq!(sum / 1000.0, 0.5, epsilon = 0.1);
    }

    #[cfg(feature = "random")]
    #[test]
    fn test_uniform() {
        for _ in 0..1000 {
            let mut rng = rand::thread_rng();
            let t = Torus::uniform(&mut rng);
            // assert!(f64::from(t) >= 0.0);
            // assert!(f64::from(t) < 1.0);
            assert!(f64::from(t) >= 0.0);
            assert!(f64::from(t) < 1.0);
        }
    }
}
