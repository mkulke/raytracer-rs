use super::vec3::Vec3;

pub struct Point3(pub Vec3);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3((x, y, z).into())
    }
}
