use rand::Rng;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;
pub type Vec3 = vec3::Vec3; // 3D vector
pub type Point3 = vec3::Vec3; // 3D point
pub type Color = vec3::Vec3; // RGB color
pub type Ray = ray::Ray;
pub type HitRecord = hittable::HitRecord;
pub type HittableList = hittable_list::HittableList;
pub type Sphere = sphere::Sphere;
pub type Camera = camera::Camera;
pub use color::{write_color, write_sampled_color};
pub use hittable::Hittable;

pub use material::Material;
pub type Lambertian = material::Lambertian;
pub type Metal = material::Metal;
pub type Reflectance = material::Reflectance;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn rand_f64() -> f64 {
    rand_in_range(0.0, 1.0)
}

// returns a random real in [min, max)
pub fn rand_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

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
