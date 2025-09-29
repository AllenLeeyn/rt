pub mod aabb;
pub mod camera;
pub mod cube;
pub mod cylinder;
pub mod color;
pub mod hit;
pub mod image;
pub mod light;
pub mod plane;
pub mod ray;
pub mod scene;
pub mod sphere;
pub mod texture;
pub mod vec3;

use std::ops::Mul;

pub fn sq<T: Mul<Output = T> + Copy>(v: T) -> T {
    v * v
}
