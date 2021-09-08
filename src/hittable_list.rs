use crate::{HitRecord, Hittable, Ray};
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn default() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object.clone());
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_rec: Option<HitRecord> = None;
        let mut curr_t_max = t_max;

        for object in self.objects.iter() {
            if let Some(obj_hit_rec) = object.hit(r, t_min, curr_t_max) {
                curr_t_max = obj_hit_rec.t;
                hit_rec = Some(obj_hit_rec);
            }
        }
        hit_rec
    }
}
