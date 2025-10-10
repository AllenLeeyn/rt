use crate::core::{Point3, Vec3, Hittable, HitRecord, Ray};
use crate::material::Material;
use crate::random_double;

pub struct Volumes {
    boundary: Box<dyn Hittable>,
    density: f32,
    phase_function: Material, // Uses your Material struct
}

impl Volumes {
    pub fn new(boundary: Box<dyn Hittable>, density: f32, phase_function: Material) -> Self {
        Self {
            boundary,
            density,
            phase_function,
        }
    }
}

impl Hittable for Volumes {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Find where the ray enters and exits the boundary object
        let hit1 = self.boundary.hit(ray, f32::NEG_INFINITY, f32::INFINITY)?;
        let hit2 = self.boundary.hit(ray, hit1.t + 0.0001, f32::INFINITY)?;

        let mut t1 = hit1.t.max(t_min);
        let mut t2 = hit2.t.min(t_max);

        if t1 >= t2 {
            return None;
        }

        t1 = t1.max(0.0); // Clamp to avoid negative distances

        let ray_length = ray.direction().length();
        let distance_inside = (t2 - t1) * ray_length;
        let hit_distance = -1.0 / self.density * random_double().ln();

        if hit_distance > distance_inside {
            return None;
        }

        let t = t1 + hit_distance / ray_length;
        let p = ray.at(t);
        let distance_from_center = (p - hit1.p).length(); // approximate center distance
        let radius = (hit2.t - hit1.t) * 0.5 * ray_length;

        let falloff = (1.0 - (distance_from_center / radius).powi(2)).clamp(0.0, 1.0); // smoother dropoff

        // You can randomly reject this sample to fake density variation
        if random_double() > falloff {
            return None;
        }

        Some(HitRecord {
            p,
            t,
            u: 0.0,
            v: 0.0,
            front_face: true,
            normal: Vec3::new(1.0, 0.0, 0.0), // Arbitrary normal (not used in volumes)
            material: self.phase_function.clone(),
            color: self.phase_function.value_at(0.0, 0.0, p),
        })
    }
}
