use crate::{clamp, rand_f64, Color, HitRecord, Ray, Vec3};

pub trait Material: Sync {
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
        let mut scatter_direction = rec.normal + Vec3::rand_unit_vector();
        // handle degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some(Reflectance {
            scattered_ray: Ray::new(rec.point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: clamp(fuzz, 0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<Reflectance> {
        let reflected = Vec3::reflect(Vec3::unit(r_in.direction()), rec.normal);
        let scattered_ray = Ray::new(
            rec.point,
            reflected + self.fuzz * Vec3::rand_in_unit_sphere(),
        );
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

pub struct Dieletric {
    index_of_refraction: f64,
}

impl Dieletric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_index: f64) -> f64 {
        // use Schlick's approximation for reflectance
        let r0 = ((1_f64 - ref_index) / (1_f64 + ref_index)).powi(2);
        r0 + (1_f64 - r0) * (1_f64 - cosine).powi(5)
    }
}

impl Material for Dieletric {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<Reflectance> {
        let refraction_ratio = if rec.is_front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = Vec3::unit(r_in.direction());

        let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dieletric::reflectance(cos_theta, refraction_ratio) > rand_f64() {
                Vec3::reflect(unit_direction, rec.normal)
            } else {
                Vec3::refract(unit_direction, rec.normal, refraction_ratio)
            };
        Some(Reflectance {
            scattered_ray: Ray::new(rec.point, direction),
            attenuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}
