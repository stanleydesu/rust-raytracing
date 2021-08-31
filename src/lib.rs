mod color;
mod ray;
mod vec3;
pub type Vec3 = vec3::Vec3; // 3D vector
pub type Point3 = vec3::Vec3; // 3D point
pub type Color = vec3::Vec3; // RGB color
pub use color::write_color;
