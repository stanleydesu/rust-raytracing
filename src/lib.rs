use std::ops::{Index, IndexMut, Neg};

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_constructor() {
        let vec = Vec3::zero();
        assert_eq!(vec.v, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn new_constructor() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.v, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn neg_operator() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let vec_arr = (-vec).v;
        let expected_arr = [-1.0, -2.0, -3.0];
        assert_eq!(vec_arr, expected_arr);
    }

    #[test]
    fn index_operator() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec[0], 1.0);
    }

    #[test]
    #[should_panic]
    fn index_operator_panic() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        vec[3];
    }

    #[test]
    fn index_mut_operator() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        let expected_arr = [-1.0, -2.0, -3.0];
        vec[0] = expected_arr[0];
        assert_eq!(vec[0], expected_arr[0]);
    }

    #[test]
    #[should_panic]
    fn index_mut_operator_panic() {
        let mut vec = Vec3::new(1.0, 2.0, 3.0);
        vec[3] = 42.0;
    }
}
