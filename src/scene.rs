use crate::color::*;
use crate::texture::*;

pub struct Scene {
    //objects: Vec<Object>,
    //lights: Vec<Light>,
    background: Texture,
    //camera: Camera,
    //max_ray_depth: u32,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            background: Texture::SolidColor(Color::new(0, 0, 0)), // black
        }
    }

    pub fn set_background(&mut self, texture: Texture) {
        self.background = texture;
    }
}