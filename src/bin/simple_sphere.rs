use raytracing::{write_color, Color, Point3, Ray, Vec3};

// given a ray and a sphere's center and radius,
// returns whether the ray hits the sphere
fn hits_sphere(r: &Ray, center: Point3, radius: f64) -> bool {
    // r = P(t) = A + tb
    // solve t²b² + 2tb(A - C) + (A - C)² - r² = 0, for some unknown t
    // which is a quadratic equation of the form ax^2 + bx + c = 0,
    // where a = b², b = 2b(A - C), and c = (A - C)² - r²
    let oc = r.origin() - center; // A - C
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(r.direction(), oc);
    let c = Vec3::dot(oc, oc) - (radius * radius);
    // negative discriminant if ray doesn't hit the sphere,
    // zero if it hits the sphere tangentially,
    // and positive if it passes through the sphere
    let discriminant = (b * b) - (4.0 * a * c);
    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hits_sphere(r, Point3::new(0.0, 0.0, -1.0), 0.5) {
        return Color::new(0.0, 1.0, 0.0);
    }
    let unit_direction = Vec3::unit(r.direction());
    // y value is now -1 <= y <= 1. Transform so 0 <= t <= 1
    let t = 0.5 * (unit_direction.y() + 1.0);
    // at t = 1: output blue
    // at t = 0: output white
    // inbetween (0 < t < 1): linear blend
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.0, 0.0, 0.0)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        -(horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    // PPM image format specifications
    println!("P3"); // colors are in ascii
    println!("{} {}", image_width, image_height);
    println!("{}", 255);

    for y in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", y + 1);
        for x in 0..image_width {
            let x_scale = x as f64 / (image_width - 1) as f64;
            let y_scale = y as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + (horizontal * x_scale) + (vertical * y_scale),
            );
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
}
