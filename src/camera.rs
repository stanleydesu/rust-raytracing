use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::zero();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            -(horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    // returns the ray pointing from the camera's origin to some location on
    // the viewport corresponding to the x/y percentage offsets
    pub fn get_ray(&self, x_percent: f64, y_percent: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner - self.origin
                + (self.horizontal * x_percent)
                + (self.vertical * y_percent),
        )
    }
}