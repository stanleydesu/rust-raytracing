use raytracing::{
    rand_f64, write_sampled_color, Camera, Color, Hittable, HittableList, Point3, Ray, Sphere, Vec3,
};
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
    let samples_per_pixel = 100u32;

    // world
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    let cam = Camera::default();

    // PPM image format specifications
    println!("P3"); // colors are in ascii
    println!("{} {}", image_width, image_height);
    println!("{}", 255);

    for y in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", y);
        for x in 0..image_width {
            let mut pixel_color = Color::zero();
            for _s in 0..samples_per_pixel {
                let x_percent = (x as f64 + rand_f64()) / (image_width as f64);
                let y_percent = (y as f64 + rand_f64()) / (image_height as f64);
                let r = cam.get_ray(x_percent, y_percent);
                pixel_color += ray_color(r, &world);
            }
            write_sampled_color(pixel_color, samples_per_pixel);
        }
    }
}
