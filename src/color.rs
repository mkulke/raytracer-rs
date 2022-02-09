use super::vec3::Vec3;
use std::cmp::PartialOrd;
use std::convert::From;
use std::error::Error;
use std::io::Write;
use std::ops;

fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

#[derive(Debug)]
pub struct Color(pub Vec3);

impl Color {
    pub fn write_sampled(
        &self,
        mut writer: impl Write,
        samples: u32,
    ) -> Result<(), Box<dyn Error>> {
        let scale = 1. / samples as f64;

        let r = (self.as_vec3().x() * scale).sqrt();
        let g = (self.as_vec3().y() * scale).sqrt();
        let b = (self.as_vec3().z() * scale).sqrt();

        let clamped_r = (256. * clamp(r, 0., 0.999)) as u8;
        let clamped_g = (256. * clamp(g, 0., 0.999)) as u8;
        let clamped_b = (256. * clamp(b, 0., 0.999)) as u8;

        writeln!(writer, "{} {} {}", clamped_r, clamped_g, clamped_b)?;
        Ok(())
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color((r, g, b).into())
    }

    pub fn as_vec3(&self) -> &Vec3 {
        &self.0
    }

    pub fn as_mut_vec3(&mut self) -> &mut Vec3 {
        &mut self.0
    }
}

impl From<Vec3> for Color {
    fn from(vec3: Vec3) -> Self {
        Self(vec3)
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.as_mut_vec3().add_assign(other.as_vec3());
    }
}
