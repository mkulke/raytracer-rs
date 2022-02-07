use super::vec3::Vec3;
use std::convert::From;
use std::error::Error;
use std::io::Write;

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

impl From<Vec3> for Color {
    fn from(vec3: Vec3) -> Self {
        Self(vec3)
    }
}
