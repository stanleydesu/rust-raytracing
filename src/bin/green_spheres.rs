use raytracing::{
    rand_f64, rand_in_range, write_sampled_color, Camera, Color, Dieletric, Hittable, HittableList,
    Lambertian, Material, Metal, Point3, Ray, Sphere, Vec3,
};
use std::sync::Arc;

fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let metal_mat = Arc::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.0));
    let metal_p = Point3::new(-4.0, 1.0, 0.5);
    world.add(Arc::new(Sphere::new(metal_p, 1.0, metal_mat)));

    let glass_mat = Arc::new(Dieletric::new(1.5));
    let glass_p = Point3::new(5.0, 1.0, 0.5);
    world.add(Arc::new(Sphere::new(glass_p, 1.0, glass_mat.clone())));
    let glass_hollow = Point3::new(5.0, 1.0, 0.5);
    world.add(Arc::new(Sphere::new(glass_hollow, -0.9, glass_mat.clone())));

    let lamber_mat = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 0.2)));
    let lamber_p = Point3::new(-1.0, 1.0, -3.0);
    world.add(Arc::new(Sphere::new(lamber_p, 1.0, lamber_mat)));

    let colors = vec![
        Color::new(0., 75., 35.) / 255.,
        Color::new(0., 100., 0.) / 255.,
        Color::new(0., 114., 0.) / 255.,
        Color::new(0., 128., 0.) / 255.,
        Color::new(56., 176., 0.) / 255.,
        Color::new(112., 224., 0.) / 255.,
        Color::new(158., 240., 26.) / 255.,
        Color::new(204., 255., 51.) / 255.,
    ];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64();
            let center = Point3::new(
                a as f64 + 0.7 * rand_f64(),
                0.2,
                b as f64 + 0.7 * rand_f64(),
            );

            let can_spawn = (center - glass_p).length() > 1.2
                && (center - lamber_p).length() > 1.2
                && (center - metal_p).length() > 1.2;

            if can_spawn {
                let sphere_material: Arc<dyn Material + Send + Sync>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = colors[(rand_in_range(0.0, colors.len() as f64)) as usize];
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else {
                    // glass
                    sphere_material = glass_mat.clone();
                }
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    world
}

fn ray_color(r: Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::zero();
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(reflectance) = rec.mat_ptr.scatter(r, rec.clone()) {
            return reflectance.attenuation
                * ray_color(reflectance.scattered_ray, world, depth - 1);
        }
        return Color::zero();
    }
    let unit_direction = Vec3::unit(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    ((1.0 - t) * Color::new(1.0, 1.0, 1.0)) + (t * Color::new(0.3, 0.7, 1.0))
}

fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200u32;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500u32;
    let max_depth = 50u32;

    // camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - Point3::new(5.0, 1.0, 0.5)).length();
    let aperture = 0.1;
    let vertical_fov = 20.0;
    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vertical_fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // PPM image format specifications
    println!("P3"); // colors are in ascii
    println!("{} {}", image_width, image_height);
    println!("{}", 255);

    // world
    let world = random_scene();

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
