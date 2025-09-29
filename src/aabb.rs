use crate::vec3::Point3;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
        let small = Point3::new(
            box0.min.x().min(box1.min.x()),
            box0.min.y().min(box1.min.y()),
            box0.min.z().min(box1.min.z()),
        );
        let big = Point3::new(
            box0.max.x().max(box1.max.x()),
            box0.max.y().max(box1.max.y()),
            box0.max.z().max(box1.max.z()),
        );
        Aabb::new(small, big)
    }
    
    pub fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let mut t0 = (self.min()[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - ray.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
