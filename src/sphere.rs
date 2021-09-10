use crate::{HitRecord, Hittable, Material, Point3, Ray, Vec3};
use std::sync::Arc;
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut hit_rec = HitRecord {
            t: root,
            point: r.at(root),
            normal: (r.at(root) - self.center) / self.radius, // unit surface normal
            is_front_face: true,
            mat_ptr: self.mat_ptr.clone(),
        };
        hit_rec.set_face_normal(r, hit_rec.normal);

        Some(hit_rec)
    }
}
