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
            let r_scale = x as f64 / image_width as f64;
            // greenness decreases from top to bottom (0.99... to 0)
            let g_scale = y as f64 / image_height as f64;
            let b_scale = 0.5 as f64;
            let r_intensity = (r_scale * color_max as f64) as u8;
            let g_intensity = (g_scale * color_max as f64) as u8;
            let b_intensity = (b_scale * color_max as f64) as u8;
            println!("{} {} {}", r_intensity, b_intensity, g_intensity);
        }
    }
}
