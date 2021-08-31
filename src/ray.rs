use crate::{Point3, Vec3};

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn assert_eq_vec3s(v1: Vec3, v2: Vec3) {
        assert_relative_eq!(v1.x(), v2.x());
        assert_relative_eq!(v1.y(), v2.y());
        assert_relative_eq!(v1.z(), v2.z());
    }

    #[test]
    fn new_constructor() {
        let v1 = Vec3::new(1.1, 2.33, 3.89);
        let v2 = Vec3::new(-1.19, 2.66, 3.77);
        let ray = Ray::new(v1, v2);
        assert_eq_vec3s(ray.origin, v1);
        assert_eq_vec3s(ray.direction, v2);
    }

    #[test]
    fn getters() {
        let v1 = Vec3::new(1.1, 2.33, 3.89);
        let v2 = Vec3::new(-1.19, 2.66, 3.77);
        let ray = Ray::new(v1, v2);
        assert_eq_vec3s(ray.origin(), v1);
        assert_eq_vec3s(ray.direction(), v2);
    }

    #[test]
    fn at() {
        let v1 = Vec3::new(1.1, 2.33, 3.89);
        let v2 = Vec3::new(-1.19, 2.66, 3.77);
        let ray = Ray::new(v1, v2);
        let t = 7.173_f64;
        assert_eq_vec3s(ray.at(t), ray.origin + t * ray.direction());
    }
}
