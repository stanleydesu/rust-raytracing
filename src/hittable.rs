use crate::{Point3, Ray, Vec3};
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
