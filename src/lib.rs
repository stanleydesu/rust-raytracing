use std::ops::{AddAssign, DivAssign, Index, IndexMut, MulAssign, Neg};

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
        self.v[0] += rhs.v[0];
        self.v[1] += rhs.v[1];
        self.v[2] += rhs.v[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.v[0] *= rhs;
        self.v[1] *= rhs;
        self.v[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1_f64 / rhs
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
    fn neg_operator() {
        let v = -(Vec3::new(1.0, 2.0, 3.0));
        let expected = Vec3::new(-1.0, -2.0, -3.0);
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
        let scalar = 5.0;
        let expected = Vec3::new(v[0] * scalar, v[1] * scalar, v[2] * scalar);
        v *= scalar;
        assert_eq_vec3s(v, expected);
    }

    #[test]
    fn div_assign_operator() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 5.0;
        let expected = Vec3::new(v[0] / scalar, v[1] / scalar, v[2] / scalar);
        v /= scalar;
        assert_eq_vec3s(v, expected);
    }
}
