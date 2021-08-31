use crate::Color;
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
