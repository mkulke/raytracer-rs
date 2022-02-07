use super::vec3::Vec3;
use std::convert::From;

pub struct Point3(Vec3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3((x, y, z).into())
    }

    pub fn as_vec3(&self) -> &Vec3 {
        &self.0
    }
}

impl From<Vec3> for Point3 {
    fn from(vec3: Vec3) -> Self {
        Self(vec3)
    }
}
