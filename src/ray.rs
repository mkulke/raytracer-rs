use super::point3::Point3;
use super::vec3::Vec3;

pub struct Ray<'a> {
    origin: &'a Point3,
    direction: Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        (self.origin.as_vec3() + &(&self.direction * t)).into()
    }

    pub fn origin(&self) -> &Point3 {
        self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}
