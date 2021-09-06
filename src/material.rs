use crate::{Color, HitRecord, Ray, Vec3};

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<Reflectance>;
}

pub struct Reflectance {
    pub attenuation: Color,
    pub scattered_ray: Ray,
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, rec: HitRecord) -> Option<Reflectance> {
        let scatter_direction = rec.point + Vec3::rand_in_hemisphere(rec.normal);
        Some(Reflectance {
            scattered_ray: Ray::new(rec.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<Reflectance> {
        let reflected = Vec3::reflect(Vec3::unit(r_in.direction()), rec.normal);
        let scattered_ray = Ray::new(rec.point, reflected);
        let attenuation = self.albedo;
        if Vec3::dot(reflected, rec.normal) > 0.0 {
            Some(Reflectance {
                scattered_ray,
                attenuation,
            })
        } else {
            None
        }
    }
}
