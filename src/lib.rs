pub mod camera;
pub mod color;
pub mod image;
pub mod ray;
pub mod scene;
pub mod texture;
pub mod vec3;

use std::ops::Mul;

pub fn sq<T: Mul<Output = T> + Copy>(v: T) -> T {
    v * v
}
