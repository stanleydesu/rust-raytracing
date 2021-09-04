use raytracing::{write_color, Color, Point3, Ray, Vec3};

// given a ray and a sphere's center and radius,
// returns -1 if the ray doesn't hit the sphere,
// else, gives a t value at which r = P(t) = A + tb hits the sphere
fn ray_hit_sphere_value(r: &Ray, center: Point3, radius: f64) -> f64 {
    let oc = r.origin() - center; // A - C
    let a = r.direction().length_squared();
    let half_b = Vec3::dot(r.direction(), oc);
    let c = oc.length_squared() - (radius * radius);
    // negative discriminant if ray doesn't hit the sphere,
    // zero if it hits the sphere tangentially,
    // and positive if it passes through the sphere
    let discriminant = (half_b * half_b) - (a * c);
    if discriminant < 0.0 {
        return -1.0;
    } else {
        // return the smaller t value (closest hit point)
        return (-half_b - discriminant.sqrt()) / a;
    };
}

fn ray_color(r: &Ray) -> Color {
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;
    let t = ray_hit_sphere_value(r, sphere_center, sphere_radius);
    if t > 0.0 {
        // surface normal unit vector at where the ray hit the sphere
        let surface_normal = Vec3::unit(r.at(t) - sphere_center);
        // map the xyz components from -1..1 to 0..1 rgb
        return 0.5
            * Color::new(
                surface_normal.x() + 1.0,
                surface_normal.y() + 1.0,
                surface_normal.z() + 1.0,
            );
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
