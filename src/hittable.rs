use crate::{Material, Point3, Ray, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material + Send + Sync>,
    pub t: f64,
    pub is_front_face: bool,
}

impl HitRecord {
    // point the hit record's normal against the ray, depending if the ray
    // hit the outside of the hittable (where is_front_face is true)
    // or the inside (is_front_face is false)
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.is_front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        if self.is_front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
