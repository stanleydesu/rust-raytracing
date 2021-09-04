mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;
pub type Vec3 = vec3::Vec3; // 3D vector
pub type Point3 = vec3::Vec3; // 3D point
pub type Color = vec3::Vec3; // RGB color
pub type Ray = ray::Ray;
pub type HitRecord = hittable::HitRecord;
pub type Sphere = sphere::Sphere;
pub use color::write_color;
pub use hittable::Hittable;

#[cfg(test)]
pub mod test_util {
    use super::*;
    use approx::assert_relative_eq;
    use proptest::prelude::prop_compose;

    // some epsilon convenient for testing...
    // approx's relative/absolute didn't work that well
    pub fn assert_f64_eq(a: f64, b: f64) {
        assert_relative_eq!(a, b, epsilon = 0.000000001);
    }

    pub fn assert_eq_vec3s(v1: Vec3, v2: Vec3) {
        assert_f64_eq(v1.x(), v2.x());
        assert_f64_eq(v1.y(), v2.y());
        assert_f64_eq(v1.z(), v2.z());
    }

    prop_compose! {
        // strategy for normal non-NaN floats
        pub fn nf64()(float in -100.0..100.0) -> f64 {
            float
        }
    }

    prop_compose! {
        pub fn arb_vec3()(x in nf64(), y in nf64(), z in nf64()) -> Vec3 {
            Vec3::new(x, y, z)
        }
    }
}
