use crate::{Point3, Vec3};

#[derive(Copy, Clone, Debug)]
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
    use crate::test_util::*;
    use proptest::prelude::{prop_compose, proptest};

    prop_compose! {
        pub fn arb_ray()(origin in arb_vec3(), direction in arb_vec3()) -> Ray {
            Ray::new(origin, direction)
        }
    }

    proptest! {
        #[test]
        fn new_constructor(v1 in arb_vec3(), v2 in arb_vec3()) {
            let ray = Ray::new(v1, v2);
            assert_eq_vec3s(ray.origin, v1);
            assert_eq_vec3s(ray.direction, v2);
        }

        #[test]
        fn getters(v1 in arb_vec3(), v2 in arb_vec3()) {
            let ray = Ray::new(v1, v2);
            assert_eq_vec3s(ray.origin(), v1);
            assert_eq_vec3s(ray.direction(), v2);
        }

        #[test]
        fn at(ray in arb_ray(), scalar in nf64()) {
            assert_eq_vec3s(ray.at(scalar), ray.origin + scalar * ray.direction());
        }
    }
}
