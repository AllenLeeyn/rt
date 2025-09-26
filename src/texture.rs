use crate::color::*;
use crate::vec3::*;

pub enum Texture {
    SolidColor(Color),
    Gradient(Color, Color, f32),
    // Checkerboard(Color, Color, f32),
    // Image(String),
}

impl Texture {
    pub fn value_at(&self, u: f32, v: f32, _point: Vec3) -> Color {
        match self {
            Texture::SolidColor(color) => *color,

            Texture::Gradient(start, end, angle_rad) => {
                let u_centered = u - 0.5;
                let v_centered = v - 0.5;

                let u_rotated = u_centered * angle_rad.cos() - v_centered * angle_rad.sin();
                let t = (u_rotated + 0.5).clamp(0.0, 1.0);
                Color::lerp(*start, *end, t)
            }
        }
    }
}
