// "hello world" of graphics - nice rainbow gradient image
// usage: cargo run --bin graphics_hello_world >> image.ppm
use raytracing::{write_color, Color};
fn main() {
    let image_width = 256;
    let image_height = 256;
    let color_max = 256;

    // PPM image format specifications
    println!("P3"); // colors are in ascii
    println!("{} {}", image_width, image_height);
    println!("{}", color_max - 1);

    for y in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", y + 1);
        for x in 0..image_width {
            // redness increases from left to right (0 to 0.99...)
            // greenness decreases from top to bottom (0.99... to 0)
            let pixel_vec = Color::new(
                x as f64 / image_width as f64,
                y as f64 / image_height as f64,
                0.7_f64,
            );
            write_color(pixel_vec);
        }
    }
}
