use crate::{degrees_to_radians, Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vertical_fov: f64, // vertical field of view angle in degrees
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit(look_from - look_at);
        let u = Vec3::unit(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    // returns the ray pointing from the camera's origin to some location on
    // the viewport corresponding to the x/y percentage offsets
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner - self.origin + (self.horizontal * s) + (self.vertical * t),
        )
    }
}
