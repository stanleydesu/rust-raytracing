use raytracing::{
    rand_f64, write_sampled_color, Camera, Color, Dieletric, Hittable, HittableList, Lambertian,
    Metal, Point3, Ray, Sphere, Vec3,
};
use std::sync::Arc;

fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth == 0 {
        return Color::zero(); // recursed ray didn't hit anything, so return black
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(reflectance) = rec.mat_ptr.scatter(r, rec.clone()) {
            return reflectance.attenuation
                * ray_color(reflectance.scattered_ray, world, depth - 1);
        }
        return Color::zero();
    }
    let unit_direction = Vec3::unit(r.direction());
    // t = y mapped to the range 0..1
    let t = 0.5 * (unit_direction.y() + 1.0);
    // linear blend of blue to white
    ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.5, 0.7, 1.0))
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100u32;
    let max_depth = 50i32;

    // world
    let mut world = HittableList::default();
    let ground_mat = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_mat = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left_mat = Arc::new(Dieletric::new(1.5));
    let right_mat = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        ground_mat,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        center_mat,
    )));

    // hollow glass sphere, using negative radius so surface normal points inwards
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        left_mat.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        left_mat.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        right_mat,
    )));

    // camera
    let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
                pixel_color += ray_color(r, &world, max_depth);
            }
            write_sampled_color(pixel_color, samples_per_pixel);
        }
    }
}
