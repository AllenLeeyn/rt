use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::color::Color;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub color: Color,       // Color from texture
    pub u: f32,             // Texture coordinate u
    pub v: f32,             // Texture coordinate v
}
 
impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }
}
 
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}
