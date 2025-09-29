use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::hit::{Hittable, HitRecord};
use crate::aabb::Aabb;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    texture: Texture,
    bounding_box: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, texture: Texture) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bounding_box = Aabb::new(center - rvec, center + rvec);
        Self {
            center,
            radius,
            texture,
            bounding_box,
        }
    }

    fn compute_uv(&self, p: Point3) -> (f32, f32) {
        let u = (p.x() - self.bounding_box.min().x()) / (self.bounding_box.max().x() - self.bounding_box.min().x());
        let v = (p.y() - self.bounding_box.min().y()) / (self.bounding_box.max().y() - self.bounding_box.min().y());
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find nearest root in [t_min, t_max]
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        let (u, v) = self.compute_uv(p);
        let color = self.texture.value_at(u, v, p);

        Some(HitRecord {
            p,
            normal,
            t: root,
            color,
            u,
            v,
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
