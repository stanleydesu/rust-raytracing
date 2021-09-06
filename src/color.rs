use crate::{clamp, Color};

// by convention, each of the red/green/blue components range from 0.0 to 1.0
// writes the translated [0,255] value of each color component
pub fn write_color(pixel_vec: Color) {
    let scaled_pixel = pixel_vec * 255.999;
    println!(
        "{} {} {}",
        scaled_pixel.x() as u8,
        scaled_pixel.y() as u8,
        scaled_pixel.z() as u8,
    );
}

pub fn write_sampled_color(pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let scaled_color = pixel_color * scale;
    println!(
        "{} {} {}",
        (256_f64 * clamp(scaled_color.x().sqrt(), 0.0, 0.999)) as u8,
        (256_f64 * clamp(scaled_color.y().sqrt(), 0.0, 0.999)) as u8,
        (256_f64 * clamp(scaled_color.z().sqrt(), 0.0, 0.999)) as u8,
    );
}
