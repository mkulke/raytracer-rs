use super::point3::Point3;
use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    origin: Point3,
    lower_left: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;
        let focal_vec3: Vec3 = (0., 0., focal_length).into();

        let origin = Point3::new(0., 0., 0.);
        let horizontal: Vec3 = (viewport_width, 0., 0.).into();
        let vertical: Vec3 = (0., viewport_height, 0.).into();
        let horizontal_split = &horizontal / 2.;
        let vertical_split = &vertical / 2.;
        let lower_left =
            (origin.as_vec3() - &horizontal_split - &vertical_split - &focal_vec3).into();
        Self {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let hu = &self.horizontal * u;
        let vv = &self.vertical * v;
        let destination = self.lower_left.as_vec3() + &hu + vv - self.origin.as_vec3();
        Ray::new(&self.origin, destination)
    }
}
