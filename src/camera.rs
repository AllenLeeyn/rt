use crate::vec3::*;

pub struct Camera {
    position: Vec3,
    look_at: Vec3,
    up: Vec3,
    fov: f32,
    aspect_ratio: f32,
    resolution: (u32, u32),
}