use crate::camera::*;
use crate::color::*;
use crate::image::*;
use crate::ray::*;
use crate::texture::*;
use crate::vec3::Vec3;

pub struct Scene {
    //objects: Vec<Object>,
    //lights: Vec<Light>,
    background: Texture,
    camera: Camera,
    //max_ray_depth: u32,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            background: Texture::SolidColor(Color::new(0, 0, 0)), // black
            camera: Camera::new(),
        }
    }

    pub fn set_background(&mut self, texture: Texture) {
        self.background = texture;
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn render(&mut self, path: &str) -> std::io::Result<()> {
        let (width, height) = self.camera().resolution();

        let mut image = Image::new(width as usize, height as usize);

        for y in 0..height {
            for x in 0..width {
                let s = (x as f32 + 0.5) / width as f32;
                let t = 1.0 - ((y as f32 + 0.5) / height as f32);

                let ray = self.camera().generate_ray(s, t);
                let color = self.ray_color(&ray, s , t);
                image.set_pixel(x as usize, y as usize, color);
            }
        }

        image.save_ppm(path)?;
        Ok(())
    }
    
    pub fn ray_color(&self, ray: &Ray, u: f32, v: f32) -> Color {

        // Later: 
        // 1. Check intersections
        // 2. Apply shading
        // 3. Return background if nothing hit

        self.background.value_at(u, v, Vec3::zero())
    }

}
