use crate::{rand_f64, rand_in_range};
use std::{
    fmt,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    pub fn new(first: f64, second: f64, third: f64) -> Self {
        Self {
            v: [first, second, third],
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }

    pub fn z(&self) -> f64 {
        self.v[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.v.iter().fold(0_f64, |total, &d| total + (d * d))
    }

    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
    }

    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self::new(
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x(),
        )
    }

    pub fn unit(vec: Self) -> Self {
        vec / vec.length()
    }

    pub fn rand() -> Self {
        Self::new(rand_f64(), rand_f64(), rand_f64())
    }

    pub fn rand_in_range(min: f64, max: f64) -> Self {
        Self::new(
            rand_in_range(min, max),
            rand_in_range(min, max),
            rand_in_range(min, max),
        )
    }

    pub fn rand_in_unit_sphere() -> Self {
        loop {
            let vec = Self::rand_in_range(-1.0, 1.0);
            if vec.length_squared() < 1.0 {
                return vec;
            }
        }
    }

    pub fn rand_unit_vector() -> Self {
        Self::unit(Self::rand_in_unit_sphere())
    }

    pub fn rand_in_hemisphere(normal: Vec3) -> Self {
        let vec = Self::rand_in_unit_sphere();
        if Self::dot(vec, normal) > 0.0 {
            vec
        } else {
            -vec
        }
    }

    pub fn rand_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(rand_in_range(-1.0, 1.0), rand_in_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    // reflects v amongst some surface where n is a unit normal vector
    pub fn reflect(v: Self, n: Self) -> Self {
        v - (2.0 * Self::dot(v, n) * n)
    }

    pub fn near_zero(&self) -> bool {
        let eps = 0.00000001;
        self.x().abs() < eps && self.y().abs() < eps && self.z().abs() < eps
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = Vec3::dot(-uv, n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt() * -n;
        r_out_perp + r_out_parallel
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1_f64 / rhs
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1_f64 / rhs)
    }
}

impl Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;
    use proptest::prelude::{prop_assert, proptest};

    // prop_assert is used when the floats involved aren't affected by certain
    // calculations and can thus be exactly compared (stricter testing).
    // otherwise, assert_eq_vec3s is used to test approximate equality between
    // Vec3s and their underlying float values
    proptest! {
        #[test]
        fn new_constructs_with_parameters(x in nf64(), y in nf64(), z in nf64()) {
            let v = Vec3::new(x, y, z);
            prop_assert!(v.v == [x, y, z]);
        }

        #[test]
        fn zero_creates_zero_vector(x in 0.0..=0.0, y in 0.0..=0.0, z in 0.0..=0.0) {
            let v = Vec3::zero();
            prop_assert!(v.v == [x, y, z]);
        }

        #[test]
        fn xyz_accesses_vec(x in nf64(), y in nf64(), z in nf64()) {
            let v = Vec3::new(x, y, z);
            prop_assert!([v.x(), v.y(), v.z()] == [x, y, z]);
        }

        #[test]
        fn neg_op_idempotent(x in nf64(), y in nf64(), z in nf64()) {
            let v = Vec3::new(x, y, z);
            prop_assert!((-(-v)).v == [x, y, z]);
        }

        #[test]
        fn neg_op_negates_vec(x in nf64(), y in nf64(), z in nf64()) {
            let v = Vec3::new(x, y, z);
            prop_assert!((-v).v == [-x, -y, -z]);
        }

        #[test]
        fn valid_subscript_indexes_vec(x in nf64(), y in nf64(), z in nf64()) {
            let v = Vec3::new(x, y, z);
            prop_assert!([v[0], v[1], v[2]] == [x, y, z]);
        }

        #[test]
        #[should_panic]
        fn invalid_subscript_panics(i in 3..100usize) {
            let v = Vec3::zero();
            v[i];
        }

        #[test]
        fn valid_mut_subscript_mutates_vec(x in nf64(), y in nf64(), z in nf64()) {
            let mut v = Vec3::zero();
            v[0] = x;
            v[1] = y;
            v[2] = z;
            prop_assert!([v[0], v[1], v[2]] == [x, y, z]);
        }

        #[test]
        #[should_panic]
        fn invalid_mut_index_panics(i in 3..100usize) {
            let mut v = Vec3::zero();
            v[i] = 0.0;
        }

        #[test]
        fn add_op_commutative(v1 in arb_vec3(), v2 in arb_vec3()) {
            prop_assert!((v1 + v2).v == (v2 + v1).v);
        }

        #[test]
        fn add_op_identity(v1 in arb_vec3()) {
            prop_assert!((v1 + Vec3::zero()).v == v1.v);
        }

        #[test]
        fn add_associative(v1 in arb_vec3(), v2 in arb_vec3(), v3 in arb_vec3()) {
            assert_eq_vec3s((v1 + v2) + v3, v1 + (v2 + v3));
        }

        #[test]
        fn add_op_correct(v1 in arb_vec3(), v2 in arb_vec3()) {
            let result = v1 + v2;
            prop_assert!(result.v == [v1.x() + v2.x(), v1.y() + v2.y(), v1.z() + v2.z()]);
        }

        #[test]
        fn sub_op_identity(v1 in arb_vec3()) {
            prop_assert!((v1 - Vec3::zero()).v == v1.v);
        }

        #[test]
        fn sub_op_correct(v1 in arb_vec3(), v2 in arb_vec3()) {
            let result = v1 - v2;
            prop_assert!(result.v == [v1.x() - v2.x(), v1.y() - v2.y(), v1.z() - v2.z()]);
        }

        #[test]
        fn mul_vec3s_commutative(v1 in arb_vec3(), v2 in arb_vec3()) {
            prop_assert!((v1 * v2).v == (v2 * v1).v);
        }

        #[test]
        fn mul_vec3s_identity(v1 in arb_vec3()) {
            prop_assert!((v1 * Vec3::new(1.0, 1.0, 1.0)).v == v1.v);
        }

        #[test]
        fn mul_vec3s_zero(v1 in arb_vec3()) {
            prop_assert!((v1 * Vec3::zero()).v == Vec3::zero().v);
        }

        #[test]
        fn mul_vec3s_associative(v1 in arb_vec3(), v2 in arb_vec3(), v3 in arb_vec3()) {
            assert_eq_vec3s((v1 * v2) * v3, v1 * (v2 * v3));
        }

        #[test]
        fn mul_vec3s_distributive(v1 in arb_vec3(), v2 in arb_vec3(), v3 in arb_vec3()) {
            assert_eq_vec3s(v1 * (v2 + v3), (v1 * v2) + (v1 * v3));
        }

        #[test]
        fn mul_vec3s_correct(v1 in arb_vec3(), v2 in arb_vec3()) {
            let expected = Vec3::new(v1.x() * v2.x(), v1.y() * v2.y(), v1.z() * v2.z());
            assert_eq_vec3s(v1 * v2, expected);
        }

        #[test]
        fn mul_scalar_identity(v1 in arb_vec3()) {
            prop_assert!((v1 * 1.0).v == v1.v);
        }

        #[test]
        fn mul_scalar_zero(v1 in arb_vec3()) {
            prop_assert!((v1 * 0.0).v == Vec3::zero().v);
        }

        #[test]
        fn mul_scalar_associative(v1 in arb_vec3(), scalar in nf64(), v2 in arb_vec3()) {
            assert_eq_vec3s((v1 * scalar) * v2, v1 * (scalar * v2));
        }

        #[test]
        fn mul_scalar_correct(v1 in arb_vec3(), scalar in nf64()) {
            let expected = Vec3::new(v1.x() * scalar, v1.y() * scalar, v1.z() * scalar);
            assert_eq_vec3s(v1 * scalar, expected);
        }

        #[test]
        fn div_scalar_identity(v1 in arb_vec3()) {
            prop_assert!((v1 / 1.0).v == v1.v);
        }

        #[test]
        fn div_scalar_correct(v1 in arb_vec3(), scalar in nf64()) {
            let expected = Vec3::new(v1.x() / scalar, v1.y() / scalar, v1.z() / scalar);
            assert_eq_vec3s(v1 / scalar, expected);
        }

        #[test]
        fn length_squared_correct(v1 in arb_vec3()) {
            let expected = v1.x() * v1.x() + v1.y() * v1.y() + v1.z() * v1.z();
            assert_f64_eq(v1.length_squared(), expected);
        }

        #[test]
        fn length_correct(v1 in arb_vec3()) {
            let expected = (v1.x() * v1.x() + v1.y() * v1.y() + v1.z() * v1.z()).sqrt();
            assert_f64_eq(v1.length(), expected);
        }
        #[test]
        fn display_correct(v1 in arb_vec3()) {
            let expected = format!("{} {} {}", v1.x(), v1.y(), v1.z());
            assert_eq!(format!("{}", v1), expected);
        }

        #[test]
        fn dot_product_commutative(v1 in arb_vec3(), v2 in arb_vec3()) {
            assert_eq!(Vec3::dot(v1, v2), Vec3::dot(v2, v1));
        }

        #[test]
        fn dot_product_correct(v1 in arb_vec3(), v2 in arb_vec3()) {
            let expected = v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z();
            assert_eq!(Vec3::dot(v1, v2), expected);
        }

        #[test]
        fn cross_product_correct(v1 in arb_vec3(), v2 in arb_vec3()) {
            let expected = Vec3::new(
                v1.y() * v2.z() - v1.z() * v2.y(),
                v1.z() * v2.x() - v1.x() * v2.z(),
                v1.x() * v2.y() - v1.y() * v2.x(),
            );
            assert_eq_vec3s(Vec3::cross(v1, v2), expected);
        }

        #[test]
        fn unit_vector_has_length_one(v1 in arb_vec3()) {
            assert_f64_eq(Vec3::unit(v1).length(), 1.0);
        }
    }
}
