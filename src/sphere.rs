use super::hit::{HitRecord, Hittable};
use super::point3::Point3;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin().as_vec3() - self.center.as_vec3();
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point.as_vec3() - self.center.as_vec3()) / self.radius;
        let rec = HitRecord::new(root, point, ray, outward_normal);

        Some(rec)
    }
}
