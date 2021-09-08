use raytracing::{write_color, Color, Hittable, HittableList, Point3, Ray, Sphere, Vec3};
use std::rc::Rc;

fn ray_color(r: Ray, h: &dyn Hittable) -> Color {
    if let Some(hit_record) = h.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = Vec3::unit(r.direction());
    // t = y mapped to the range 0..1
    let t = 0.5 * (unit_direction.y() + 1.0);
    // linear blend of blue to white
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // world
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
            let pixel_color = ray_color(r, &world);
            write_color(pixel_color);
        }
    }
}
