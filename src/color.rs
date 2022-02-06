use super::vec3::Vec3;
use std::error::Error;
use std::io::Write;
use std::ops;

#[derive(Debug)]
pub struct Color(pub Vec3);

impl Color {
    pub fn write(&self, mut writer: impl Write) -> Result<(), Box<dyn Error>> {
        let ir = (255.999 * self.0.x()) as u8;
        let ig = (255.999 * self.0.y()) as u8;
        let ib = (255.999 * self.0.z()) as u8;

        writeln!(writer, "{} {} {}", ir, ig, ib)?;
        Ok(())
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color((r, g, b).into())
    }
}

impl ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, t: f64) -> Self::Output {
        Color(&self.0 * t)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, t: f64) -> Self::Output {
        &self * t
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color(self.0 + other.0)
    }
}
