use crate::color::*;
use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::*;

use rand::Rng;

pub struct Light {
    pub position: Point3,
    pub color: Color,
    pub intensity: f32,
    pub samples: usize,
    pub radius: f32,
    pub softness: f32,
}

impl Light {
    pub fn new(
        position: Point3,
        color: Color,
        intensity: f32,
        samples: usize,
        radius: f32,
        softness: f32,
    ) -> Self {

        Self {
            position,
            color,
            intensity,
            samples,
            radius,
            softness,
        }
    }
    
    /// Returns normalized direction from hit point to light
    pub fn direction_from(&self, point: Point3) -> Vec3 {
        (self.position - point).unit()
    }

    /// Returns squared distance from point to light (saves sqrt cost)
    pub fn distance_squared(&self, point: Point3) -> f32 {
        (self.position - point).length_squared()
    }

    /// Returns distance from point to light
    pub fn distance(&self, point: Point3) -> f32 {
        (self.position - point).length()
    }

    /// Computes diffuse light contribution using Lambertian reflection
    pub fn diffuse(&self, normal: Vec3, light_dir: Vec3) -> f32 {
        normal.dot(light_dir).max(0.0)
    }

    /// Returns light attenuation factor (can expand this later)
    pub fn attenuation(&self, point: Point3) -> f32 {
        let dist2 = self.distance_squared(point);
        self.intensity / dist2
    }

    pub fn random_point_on_light(&self) -> Point3 {
        let radius = self.radius * (self.softness.sqrt() + 1e-3);
        let mut rng = rand::rng();
        let theta = rng.random_range(0.0..2.0 * std::f32::consts::PI);
        let r = rng.random_range(0.0..radius);
        let dx = r * theta.cos();
        let dz = r * theta.sin();
        self.position + Vec3::new(dx, 0.0, dz) // area light on XZ plane
    }

    pub fn contribution_from_hit(
        &self,
        objects: &[Box<dyn Hittable>],
        hit: &HitRecord,
    ) -> Color {
        let mut visible = 0;
        for _ in 0..self.samples {

            let sample_point = self.random_point_on_light();
            let light_dir = (sample_point - hit.p).unit();
            let light_dist = (sample_point - hit.p).length();

            let shadow_origin = hit.p + hit.normal * 1e-3;
            let shadow_ray = Ray::new(shadow_origin, light_dir);

            let in_shadow = objects.iter().any(|obj| {
                obj.hit(&shadow_ray, 1e-3, light_dist).is_some()
            });

            if !in_shadow {
                visible += 1;
            }
        }

        let visibility = visible as f32 / self.samples as f32;

        let main_light_dir = self.direction_from(hit.p);
        let diffuse = self.diffuse(hit.normal, main_light_dir);
        let attenuation = self.attenuation(hit.p);

        hit.color * self.color * (diffuse * attenuation * visibility)
    }

}
