use super::color::Color;
use super::point3::Point3;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Point3, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = Vec3::dot(ray.direction(), &outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut potential_hit: Option<HitRecord> = None;

        for object in self.iter() {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                potential_hit = Some(rec);
            }
        }

        potential_hit
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn ray_color(&self, ray: &Ray, depth: u8) -> Color {
        if depth == 0 {
            return Color::new(0., 0., 0.);
        }

        if let Some(rec) = self.hit(ray, 0.001, f64::INFINITY) {
            let target = rec.p.as_vec3() + &rec.normal + Vec3::random_unit_vector();
            let ray = Ray::new(&rec.p, target - rec.p.as_vec3());
            return (self.ray_color(&ray, depth - 1).as_vec3() * 0.5).into();
        }
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        let vec3_a = Vec3::new(1., 1., 1.) * (1. - t);
        let vec3_b = Vec3::new(0.5, 0.7, 1.) * t;
        (vec3_a + vec3_b).into()
    }
}
