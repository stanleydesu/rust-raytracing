use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

#[derive(Copy, Clone)]
pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    pub fn new(first: f64, second: f64, third: f64) -> Vec3 {
        Vec3 {
            v: [first, second, third],
        }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
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

    pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
        lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
    }

    pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x(),
        )
    }

    pub fn unit(vec: Vec3) -> Vec3 {
        vec / vec.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
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
    fn add_assign(&mut self, rhs: Vec3) {
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
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn assert_eq_vec3s(v1: Vec3, v2: Vec3) {
        assert_relative_eq!(v1.v[0], v2.v[0]);
        assert_relative_eq!(v1.v[1], v2.v[1]);
        assert_relative_eq!(v1.v[2], v2.v[2]);
    }

    #[test]
    fn default_constructor() {
        let v = Vec3::zero();
        assert_eq!(v.v, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn new_constructor() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.v, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn xyz_accessors() {
        let values = [1.0, 2.4, 3.9];
        let v = Vec3::new(values[0], values[1], values[2]);
        assert_relative_eq!(v.x(), values[0]);
        assert_relative_eq!(v.y(), values[1]);
        assert_relative_eq!(v.z(), values[2]);
    }

    #[test]
    fn neg_operator() {
        let v = -(Vec3::new(1.0042, 2.3332, 3.141242));
        let expected = Vec3::new(-1.0042, -2.3332, -3.141242);
        assert_eq_vec3s(v, expected);
    }

    #[test]
    fn index_operator() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_relative_eq!(v[0], 1.0);
    }

    #[test]
    #[should_panic]
    fn index_operator_panic() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        v[3];
    }

    #[test]
    fn index_mut_operator() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let expected = Vec3::new(-1.0, -2.0, -3.0);
        v[0] = expected[0];
        assert_relative_eq!(v[0], expected[0]);
    }

    #[test]
    #[should_panic]
    fn index_mut_operator_panic() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v[3] = 42.0;
    }

    #[test]
    fn add_assign_operator() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(-1.0, 2.5, 3.6);
        let expected = Vec3::new(v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]);
        v1 += v2;
        assert_eq_vec3s(v1, expected);
    }

    #[test]
    fn mul_assign_operator() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 42.3116;
        let expected = Vec3::new(v[0] * scalar, v[1] * scalar, v[2] * scalar);
        v *= scalar;
        assert_eq_vec3s(v, expected);
    }

    #[test]
    fn div_assign_operator() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 0.91;
        let expected = Vec3::new(v[0] / scalar, v[1] / scalar, v[2] / scalar);
        v /= scalar;
        assert_eq_vec3s(v, expected);
    }

    #[test]
    fn length_squared() {
        let v = Vec3::new(1.1, 2.33, 3.89);
        let expected = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
        assert_relative_eq!(v.length_squared(), expected);
    }

    #[test]
    fn length() {
        let v = Vec3::new(1.1, 2.33, 3.89);
        let expected = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        assert_relative_eq!(v.length(), expected);
    }

    #[test]
    fn display() {
        let v = Vec3::new(1.1, 2.29, 4.2);
        let expected = "1.1 2.29 4.2";
        assert_eq!(format!("{}", v), expected);
    }

    #[test]
    fn add_operator() {
        let v1 = Vec3::new(1.1, 2.33, 3.89);
        let v2 = Vec3::new(-1.19, 2.66, 3.77);
        let expected = Vec3::new(v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]);
        assert_eq_vec3s(v1 + v2, expected);
    }

    #[test]
    fn sub_operator() {
        let v1 = Vec3::new(1.1, 2.33, 3.89);
        let v2 = Vec3::new(-1.19, 2.66, 3.77);
        let expected = Vec3::new(v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]);
        assert_eq_vec3s(v1 - v2, expected);
    }

    #[test]
    fn mul_vec3s_operator() {
        let v1 = Vec3::new(1.1, 2.33, 3.89);
        let v2 = Vec3::new(-1.19, 2.66, 3.77);
        let expected = Vec3::new(v1[0] * v2[0], v1[1] * v2[1], v1[2] * v2[2]);
        assert_eq_vec3s(v1 * v2, expected);
    }

    #[test]
    fn mul_scalar_operator() {
        let v = Vec3::new(1.1, 2.33, 3.89);
        let scalar = 0.49;
        let expected = Vec3::new(v[0] * scalar, v[1] * scalar, v[2] * scalar);
        assert_eq_vec3s(v * scalar, expected);
        assert_eq_vec3s(scalar * v, expected);
    }

    #[test]
    fn div_scalar_operator() {
        let v = Vec3::new(1.1, 2.33, 3.89);
        let scalar = 0.49;
        let expected = Vec3::new(v[0] / scalar, v[1] / scalar, v[2] / scalar);
        assert_eq_vec3s(v / scalar, expected);
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(1.1, 2.33, 3.89);
        let v2 = Vec3::new(-1.19, 2.66, 3.77);
        let expected = v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
        assert_eq!(Vec3::dot(v1, v2), expected);
    }

    #[test]
    fn cross_product() {
        let v1 = Vec3::new(1.1, 2.33, 3.89);
        let v2 = Vec3::new(-1.19, 2.66, 3.77);
        let expected = Vec3::new(
            v1[1] * v2[2] - v1[2] * v2[1],
            v1[2] * v2[0] - v1[0] * v2[2],
            v1[0] * v2[1] - v1[1] * v2[0],
        );
        assert_eq_vec3s(Vec3::cross(v1, v2), expected);
    }

    #[test]
    fn unit_vector() {
        let v = Vec3::new(1.1, 2.33, 3.89);
        assert_relative_eq!(Vec3::unit(v).length(), 1_f64);
    }
}